use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Datelike, Timelike, Utc};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::term;
use ecow::eco_format;
use parking_lot::RwLock;
use pathdiff::diff_paths;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use typst::diag::{bail, At, Severity, SourceDiagnostic, SourceResult, StrResult, Warned};
use typst::foundations::{Datetime, Smart};
use typst::html::HtmlDocument;
use typst::layout::{Frame, Page, PageRanges, PagedDocument};
use typst::syntax::{FileId, Source, Span};
use typst::WorldExt;
use typst_pdf::{PdfOptions, PdfStandards, Timestamp};

use super::args::{CompileArgs, DiagnosticFormat, OutputFormat};
use super::package::downloader;
use super::timings::Timer;
use super::watch::Status;
use super::world::SystemWorld;

type CodespanResult<T> = Result<T, CodespanError>;
type CodespanError = codespan_reporting::files::Error;

/// Execute a compilation command.
pub fn compile(timer: &mut Timer, args: &CompileArgs) -> StrResult<()> {
    let mut config = CompileConfig::new(args)?;
    let mut world = SystemWorld::new(&args.input, &args.world, &args.process)
        .map_err(|err| eco_format!("{err}"))?;
    timer.record(&mut world, |world| compile_once(world, &mut config))?
}

/// A preprocessed `CompileCommand`.
pub struct CompileConfig {
    /// Path to input Typst file or stdin.
    pub input: PathBuf,
    /// Path to output file (PDF, PNG, SVG, or HTML).
    pub output: PathBuf,
    /// The format of the output file.
    pub output_format: OutputFormat,
    /// Which pages to export.
    pub pages: Option<PageRanges>,
    /// The document's creation date formatted as a UNIX timestamp, with UTC suffix.
    pub creation_timestamp: Option<DateTime<Utc>>,
    /// The format to emit diagnostics in.
    pub diagnostic_format: DiagnosticFormat,
    /// Opens the output file with the default viewer or a specific program after
    /// compilation.
    pub open: Option<Option<String>>,
    /// One (or multiple comma-separated) PDF standards that Typst will enforce
    /// conformance with.
    pub pdf_standards: PdfStandards,
    /// A path to write a Makefile rule describing the current compilation.
    pub make_deps: Option<PathBuf>,
    /// The PPI (pixels per inch) to use for PNG export.
    pub ppi: f32,
    /// The export cache for images, used for caching output files in `typst
    pub export_cache: ExportCache,
}

impl CompileConfig {
    /// Preprocess a `CompileCommand`, producing a compilation config.
    pub fn new(args: &CompileArgs) -> StrResult<Self> {
        Self::new_impl(args)
    }

    /// The shared implementation of [`CompileConfig::new`] and
    /// [`CompileConfig::watching`].
    fn new_impl(args: &CompileArgs) -> StrResult<Self> {
        let input = args.input.clone();

        let output_format = if let Some(specified) = args.format {
            specified
        } else if let Some(output) = &args.output {
            match output.extension() {
                Some(ext) if ext.eq_ignore_ascii_case("pdf") => OutputFormat::Pdf,
                Some(ext) if ext.eq_ignore_ascii_case("png") => OutputFormat::Png,
                Some(ext) if ext.eq_ignore_ascii_case("svg") => OutputFormat::Svg,
                Some(ext) if ext.eq_ignore_ascii_case("html") => OutputFormat::Html,
                _ => {
                    return Err(eco_format!(
                        "could not infer output format for path {}.\n\
                     consider providing the format manually with `--format/-f`",
                        output.display()
                    ))
                }
            }
        } else {
            OutputFormat::Pdf
        };

        let output = args.output.clone().unwrap_or_else(|| {
            input.with_extension(match output_format {
                OutputFormat::Pdf => "pdf",
                OutputFormat::Png => "png",
                OutputFormat::Svg => "svg",
                OutputFormat::Html => "html",
            })
        });

        let pages = args.pages.as_ref().map(|export_ranges| {
            PageRanges::new(export_ranges.iter().map(|r| r.0.clone()).collect())
        });

        let pdf_standards = PdfStandards::new(&args.pdf_standard)?;

        Ok(Self {
            input,
            output,
            output_format,
            pages,
            pdf_standards,
            creation_timestamp: args.world.creation_timestamp,
            make_deps: args.make_deps.clone(),
            ppi: args.ppi,
            diagnostic_format: args.process.diagnostic_format,
            open: args.open.clone(),
            export_cache: ExportCache::new(),
        })
    }
}

/// Compile a single time.
///
/// Returns whether it compiled without errors.
#[typst_macros::time(name = "compile once")]
pub fn compile_once(world: &mut SystemWorld, config: &mut CompileConfig) -> StrResult<()> {
    let start = std::time::Instant::now();

    let Warned { output, warnings } = compile_and_export(world, config);

    match output {
        // Export the PDF / PNG.
        Ok(outputs) => {
            let duration = start.elapsed();

            print_diagnostics(world, &[], &warnings, config.diagnostic_format)
                .map_err(|err| eco_format!("failed to print diagnostics ({err})"))?;

            write_make_deps(world, config, outputs)?;
            open_output(config)?;
        }

        // Print diagnostics.
        Err(errors) => {
            print_diagnostics(world, &errors, &warnings, config.diagnostic_format)
                .map_err(|err| eco_format!("failed to print diagnostics ({err})"))?;
        }
    }

    Ok(())
}

/// Compile and then export the document.
fn compile_and_export(
    world: &mut SystemWorld,
    config: &mut CompileConfig,
) -> Warned<SourceResult<Vec<PathBuf>>> {
    match config.output_format {
        OutputFormat::Html => {
            let Warned { output, warnings } = typst::compile::<HtmlDocument>(world);
            let result = output.and_then(|document| export_html(&document, config));
            Warned {
                output: result.map(|()| vec![config.output.clone()]),
                warnings,
            }
        }
        _ => {
            let Warned { output, warnings } = typst::compile::<PagedDocument>(world);
            let result = output.and_then(|document| export_paged(&document, config));
            Warned {
                output: result,
                warnings,
            }
        }
    }
}

/// Export to HTML.
fn export_html(document: &HtmlDocument, config: &CompileConfig) -> SourceResult<()> {
    let html = typst_html::html(document)?;
    let result = write_to_path(&config.output, html.as_bytes());

    result
        .map_err(|err| eco_format!("failed to write HTML file ({err})"))
        .at(Span::detached())
}

/// Export to a paged target format.
fn export_paged(document: &PagedDocument, config: &CompileConfig) -> SourceResult<Vec<PathBuf>> {
    match config.output_format {
        OutputFormat::Pdf => export_pdf(document, config).map(|()| vec![config.output.clone()]),
        OutputFormat::Png => {
            export_image(document, config, ImageExportFormat::Png).at(Span::detached())
        }
        OutputFormat::Svg => {
            export_image(document, config, ImageExportFormat::Svg).at(Span::detached())
        }
        OutputFormat::Html => unreachable!(),
    }
}

/// Export to a PDF.
fn export_pdf(document: &PagedDocument, config: &CompileConfig) -> SourceResult<()> {
    // If the timestamp is provided through the CLI, use UTC suffix,
    // else, use the current local time and timezone.
    let timestamp = match config.creation_timestamp {
        Some(timestamp) => convert_datetime(timestamp).map(Timestamp::new_utc),
        None => {
            let local_datetime = chrono::Local::now();
            convert_datetime(local_datetime).and_then(|datetime| {
                Timestamp::new_local(datetime, local_datetime.offset().local_minus_utc() / 60)
            })
        }
    };
    let options = PdfOptions {
        ident: Smart::Auto,
        timestamp,
        page_ranges: config.pages.clone(),
        standards: config.pdf_standards.clone(),
    };
    let buffer = typst_pdf::pdf(document, &options)?;
    write_to_path(&config.output, &buffer)
        .map_err(|err| eco_format!("failed to write PDF file ({err})"))
        .at(Span::detached())?;
    Ok(())
}

/// Convert [`chrono::DateTime`] to [`Datetime`]
fn convert_datetime<Tz: chrono::TimeZone>(date_time: chrono::DateTime<Tz>) -> Option<Datetime> {
    Datetime::from_ymd_hms(
        date_time.year(),
        date_time.month().try_into().ok()?,
        date_time.day().try_into().ok()?,
        date_time.hour().try_into().ok()?,
        date_time.minute().try_into().ok()?,
        date_time.second().try_into().ok()?,
    )
}

/// An image format to export in.
#[derive(Clone, Copy)]
enum ImageExportFormat {
    Png,
    Svg,
}

/// Export to one or multiple images.
fn export_image(
    document: &PagedDocument,
    config: &CompileConfig,
    fmt: ImageExportFormat,
) -> StrResult<Vec<PathBuf>> {
    // Determine whether we have indexable templates in output
    let can_handle_multiple =
        output_template::has_indexable_template(config.output.to_str().unwrap_or_default());

    let exported_pages = document
        .pages
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            config.pages.as_ref().map_or(true, |exported_page_ranges| {
                exported_page_ranges.includes_page_index(*i)
            })
        })
        .collect::<Vec<_>>();

    if !can_handle_multiple && exported_pages.len() > 1 {
        let err = "without a page number template ({p}, {0p}) in the output path";
        bail!("cannot export multiple images {}", err);
    }

    // The results are collected in a `Vec<()>` which does not allocate.
    exported_pages
        .par_iter()
        .map(|(i, page)| {
            // Use output with converted path.
            let output = {
                let storage;
                let path = if can_handle_multiple {
                    storage = output_template::format(
                        config.output.to_str().unwrap_or_default(),
                        i + 1,
                        document.pages.len(),
                    );
                    Path::new(&storage)
                } else {
                    &config.output
                };

                path.to_owned()
            };

            export_image_page(config, page, &output, fmt)?;
            Ok(output)
        })
        .collect::<StrResult<Vec<PathBuf>>>()
}

/// Export single image.
fn export_image_page(
    config: &CompileConfig,
    page: &Page,
    output: &PathBuf,
    fmt: ImageExportFormat,
) -> StrResult<()> {
    match fmt {
        ImageExportFormat::Png => {
            let pixmap = typst_render::render(page, config.ppi / 72.0);
            let buf = pixmap
                .encode_png()
                .map_err(|err| eco_format!("failed to encode PNG file ({err})"))?;
            write_to_path(output, &buf)
                .map_err(|err| eco_format!("failed to write PNG file ({err})"))?;
        }
        ImageExportFormat::Svg => {
            let svg = typst_svg::svg(page);
            write_to_path(output, svg.as_bytes())
                .map_err(|err| eco_format!("failed to write SVG file ({err})"))?;
        }
    }
    Ok(())
}

pub fn write_to_path(path: &PathBuf, buffer: &[u8]) -> StrResult<()> {
    fs::write(path, buffer).map_err(|err| eco_format!("{err}"))
}

/// Caches exported files so that we can avoid re-exporting them if they haven't
/// changed.
///
/// This is done by having a list of size `files.len()` that contains the hashes
/// of the last rendered frame in each file. If a new frame is inserted, this
/// will invalidate the rest of the cache, this is deliberate as to decrease the
/// complexity and memory usage of such a cache.
pub struct ExportCache {
    /// The hashes of last compilation's frames.
    pub cache: RwLock<Vec<u128>>,
}

impl ExportCache {
    /// Creates a new export cache.
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(Vec::with_capacity(32)),
        }
    }

    /// Returns true if the entry is cached and appends the new hash to the
    /// cache (for the next compilation).
    pub fn is_cached(&self, i: usize, frame: &Frame) -> bool {
        let hash = typst::utils::hash128(frame);

        let mut cache = self.cache.upgradable_read();
        if i >= cache.len() {
            cache.with_upgraded(|cache| cache.push(hash));
            return false;
        }

        cache.with_upgraded(|cache| std::mem::replace(&mut cache[i], hash) == hash)
    }
}
/// Writes a Makefile rule describing the relationship between the output and
/// its dependencies to the path specified by the --make-deps argument, if it
/// was provided.
fn write_make_deps(
    world: &mut SystemWorld,
    config: &CompileConfig,
    outputs: Vec<PathBuf>,
) -> StrResult<()> {
    let Some(ref make_deps_path) = config.make_deps else {
        return Ok(());
    };
    let Ok(output_paths) = outputs
        .into_iter()
        .map(|o| o.into_os_string().into_string())
        .collect::<Result<Vec<_>, _>>()
    else {
        bail!("failed to create make dependencies file because output path was not valid unicode")
    };
    if output_paths.is_empty() {
        bail!("failed to create make dependencies file because output was stdout")
    }

    fn munge(s: &str) -> String {
        let mut res = String::with_capacity(s.len());
        let mut slashes = 0;
        for c in s.chars() {
            match c {
                '\\' => slashes += 1,
                '$' => {
                    res.push('$');
                    slashes = 0;
                }
                ':' => {
                    res.push('\\');
                    slashes = 0;
                }
                ' ' | '\t' => {
                    for _ in 0..slashes + 1 {
                        res.push('\\');
                    }
                    slashes = 0;
                }
                '#' => {
                    res.push('\\');
                    slashes = 0;
                }
                _ => slashes = 0,
            };
            res.push(c);
        }
        res
    }

    fn write(
        make_deps_path: &Path,
        output_paths: Vec<String>,
        root: PathBuf,
        dependencies: impl Iterator<Item = PathBuf>,
    ) -> io::Result<()> {
        let mut file = File::create(make_deps_path)?;
        let current_dir = std::env::current_dir()?;
        let relative_root = diff_paths(&root, &current_dir).unwrap_or(root.clone());

        for (i, output_path) in output_paths.into_iter().enumerate() {
            if i != 0 {
                file.write_all(b" ")?;
            }
            file.write_all(munge(&output_path).as_bytes())?;
        }
        file.write_all(b":")?;
        for dependency in dependencies {
            let relative_dependency = match dependency.strip_prefix(&root) {
                Ok(root_relative_dependency) => relative_root.join(root_relative_dependency),
                Err(_) => dependency,
            };
            let Some(relative_dependency) = relative_dependency.to_str() else {
                continue;
            };

            file.write_all(b" ")?;
            file.write_all(munge(relative_dependency).as_bytes())?;
        }
        file.write_all(b"\n")?;

        Ok(())
    }

    write(
        make_deps_path,
        output_paths,
        world.root().to_owned(),
        world.dependencies(),
    )
    .map_err(|err| eco_format!("failed to create make dependencies file due to IO error ({err})"))
}

/// Opens the output if desired.
fn open_output(config: &mut CompileConfig) -> StrResult<()> {
    let Some(viewer) = config.open.take() else {
        return Ok(());
    };

    // Some resource openers require the path to be canonicalized.
    let path = config
        .output
        .canonicalize()
        .map_err(|err| eco_format!("failed to canonicalize path ({err})"))?;

    open_path(path.as_os_str(), viewer.as_deref())
}

/// Opens the given file using:
///
/// - The default file viewer if `app` is `None`.
/// - The given viewer provided by `app` if it is `Some`.
fn open_path(path: &OsStr, viewer: Option<&str>) -> StrResult<()> {
    if let Some(viewer) = viewer {
        open::with_detached(path, viewer)
            .map_err(|err| eco_format!("failed to open file with {} ({})", viewer, err))
    } else {
        open::that_detached(path).map_err(|err| {
            let openers = open::commands(path)
                .iter()
                .map(|command| command.get_program().to_string_lossy())
                .collect::<Vec<_>>()
                .join(", ");
            eco_format!(
                "failed to open file with any of these resource openers: {} ({})",
                openers,
                err,
            )
        })
    }
}

/// Print diagnostic messages to the terminal.
pub fn print_diagnostics(
    world: &SystemWorld,
    errors: &[SourceDiagnostic],
    warnings: &[SourceDiagnostic],
    diagnostic_format: DiagnosticFormat,
) -> Result<(), codespan_reporting::files::Error> {
    let mut config = term::Config {
        tab_width: 2,
        ..Default::default()
    };
    if diagnostic_format == DiagnosticFormat::Short {
        config.display_style = term::DisplayStyle::Short;
    }

    for diagnostic in warnings.iter().chain(errors) {
        let diag = match diagnostic.severity {
            Severity::Error => Diagnostic::error(),
            Severity::Warning => Diagnostic::warning(),
        }
        .with_message(diagnostic.message.clone())
        .with_notes(
            diagnostic
                .hints
                .iter()
                .map(|e| (eco_format!("hint: {e}")).into())
                .collect(),
        )
        .with_labels(label(world, diagnostic.span).into_iter().collect());

        // term::emit(&mut terminal::out(), &config, world, &diag)?;

        // Stacktrace-like helper diagnostics.
        for point in &diagnostic.trace {
            let message = point.v.to_string();
            let help = Diagnostic::help()
                .with_message(message)
                .with_labels(label(world, point.span).into_iter().collect());

            // term::emit(&mut terminal::out(), &config, world, &help)?;
        }
    }

    Ok(())
}

/// Create a label for a span.
fn label(world: &SystemWorld, span: Span) -> Option<Label<FileId>> {
    Some(Label::primary(span.id()?, world.range(span)?))
}

impl<'a> codespan_reporting::files::Files<'a> for SystemWorld {
    type FileId = FileId;
    type Name = String;
    type Source = Source;

    fn name(&'a self, id: FileId) -> CodespanResult<Self::Name> {
        let vpath = id.vpath();
        Ok(if let Some(package) = id.package() {
            format!("{package}{}", vpath.as_rooted_path().display())
        } else {
            // Try to express the path relative to the working directory.
            vpath
                .resolve(self.root())
                .and_then(|abs| pathdiff::diff_paths(abs, self.workdir()))
                .as_deref()
                .unwrap_or_else(|| vpath.as_rootless_path())
                .to_string_lossy()
                .into()
        })
    }

    fn source(&'a self, id: FileId) -> CodespanResult<Self::Source> {
        Ok(self.lookup(id))
    }

    fn line_index(&'a self, id: FileId, given: usize) -> CodespanResult<usize> {
        let source = self.lookup(id);
        source
            .byte_to_line(given)
            .ok_or_else(|| CodespanError::IndexTooLarge {
                given,
                max: source.len_bytes(),
            })
    }

    fn line_range(&'a self, id: FileId, given: usize) -> CodespanResult<std::ops::Range<usize>> {
        let source = self.lookup(id);
        source
            .line_to_range(given)
            .ok_or_else(|| CodespanError::LineTooLarge {
                given,
                max: source.len_lines(),
            })
    }

    fn column_number(&'a self, id: FileId, _: usize, given: usize) -> CodespanResult<usize> {
        let source = self.lookup(id);
        source.byte_to_column(given).ok_or_else(|| {
            let max = source.len_bytes();
            if given <= max {
                CodespanError::InvalidCharBoundary { given }
            } else {
                CodespanError::IndexTooLarge { given, max }
            }
        })
    }
}

mod output_template {
    const INDEXABLE: [&str; 3] = ["{p}", "{0p}", "{n}"];

    pub fn has_indexable_template(output: &str) -> bool {
        INDEXABLE.iter().any(|template| output.contains(template))
    }

    pub fn format(output: &str, this_page: usize, total_pages: usize) -> String {
        // Find the base 10 width of number `i`
        fn width(i: usize) -> usize {
            1 + i.checked_ilog10().unwrap_or(0) as usize
        }

        let other_templates = ["{t}"];
        INDEXABLE
            .iter()
            .chain(other_templates.iter())
            .fold(output.to_string(), |out, template| {
                let replacement = match *template {
                    "{p}" => format!("{this_page}"),
                    "{0p}" | "{n}" => format!("{:01$}", this_page, width(total_pages)),
                    "{t}" => format!("{total_pages}"),
                    _ => unreachable!("unhandled template placeholder {template}"),
                };
                out.replace(template, replacement.as_str())
            })
    }
}
