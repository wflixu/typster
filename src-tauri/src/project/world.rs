use chrono::{DateTime, Datelike, FixedOffset, Local, Utc};
use log::{debug, info};
use parking_lot::{Mutex, RwLock};
use std::cell::{OnceCell, RefCell, RefMut};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, OnceLock};
use std::{fmt, fs, io, mem};
use typst::diag::{FileError, FileResult, PackageError, PackageResult};
use typst::foundations::{Bytes, Datetime};
use typst::layout::Frame;
use typst::syntax::package::PackageSpec;
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, World};
use typst_kit::fonts::{FontSearcher, FontSlot, Fonts};
use typst_kit::package::PackageStorage;
use typst_timing::timed;

use super::package::{self, PrintDownload};

use super::{download_package, ProjectConfig};



/// An error that occurs during world construction.
#[derive(Debug)]
pub enum WorldCreationError {
    /// The input file does not appear to exist.
    InputNotFound(PathBuf),
    /// The input file is not contained within the root folder.
    InputOutsideRoot,
    /// The root directory does not appear to exist.
    RootNotFound(PathBuf),
    /// Another type of I/O error.
    Io(io::Error),
}
impl fmt::Display for WorldCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorldCreationError::InputNotFound(path) => {
                write!(f, "input file not found (searched at {})", path.display())
            }
            WorldCreationError::InputOutsideRoot => {
                write!(f, "source file must be contained in project root")
            }
            WorldCreationError::RootNotFound(path) => {
                write!(
                    f,
                    "root directory not found (searched at {})",
                    path.display()
                )
            }
            WorldCreationError::Io(err) => write!(f, "{err}"),
        }
    }
}

/// Lazily processes data for a file.
struct SlotCell<T> {
    /// The processed data.
    data: Option<FileResult<T>>,
    /// A hash of the raw file contents / access error.
    fingerprint: u128,
    /// Whether the slot has been accessed in the current compilation.
    accessed: bool,
}

/// The current date and time.
enum Now {
    /// The date and time if the environment `SOURCE_DATE_EPOCH` is set.
    /// Used for reproducible builds.
    Fixed(DateTime<Utc>),
    /// The current date and time if the time is not externally fixed.
    System(OnceLock<DateTime<Utc>>),
}

struct FileSlot {
    /// The slot's file id.
    id: FileId,
    /// The lazily loaded and incrementally updated source file.
    source: SlotCell<Source>,
    /// The lazily loaded raw byte buffer.
    file: SlotCell<Bytes>,
}

impl FileSlot {
    /// Create a new file slot.
    fn new(id: FileId) -> Self {
        Self {
            id,
            file: SlotCell::new(),
            source: SlotCell::new(),
        }
    }

    /// Whether the file was accessed in the ongoing compilation.
    fn accessed(&self) -> bool {
        self.source.accessed() || self.file.accessed()
    }

    /// Marks the file as not yet accessed in preparation of the next
    /// compilation.
    fn reset(&mut self) {
        self.source.reset();
        self.file.reset();
    }

    /// Retrieve the source for this file.
    fn source(
        &mut self,
        project_root: &Path,
        package_storage: &PackageStorage,
    ) -> FileResult<Source> {
        info!("fn source: {:?}", self.id.vpath());
        self.source.get_or_init(
            || read(self.id, project_root, package_storage),
            |data, prev| {
                let text = decode_utf8(&data)?;
                if let Some(mut prev) = prev {
                    prev.replace(text);
                    Ok(prev)
                } else {
                    Ok(Source::new(self.id, text.into()))
                }
            },
        )
    }

    /// Retrieve the file's bytes.
    fn file(&mut self, project_root: &Path, package_storage: &PackageStorage) -> FileResult<Bytes> {
        info!("fn file: {:?}", self.id.vpath());
        self.file.get_or_init(
            || read(self.id, project_root, package_storage),
            |data, _| Ok(data.into()),
        )
    }
}

/// Decode UTF-8 with an optional BOM.
fn decode_utf8(buf: &[u8]) -> FileResult<&str> {
    // Remove UTF-8 BOM.
    Ok(std::str::from_utf8(
        buf.strip_prefix(b"\xef\xbb\xbf").unwrap_or(buf),
    )?)
}

/// Resolves the path of a file id on the system, downloading a package if
/// necessary.
fn system_path(
    project_root: &Path,
    id: FileId,
    package_storage: &PackageStorage,
) -> FileResult<PathBuf> {
    // Determine the root path relative to which the file path
    // will be resolved.
    let buf;
    let mut root = project_root;
    if let Some(spec) = id.package() {
        buf = package_storage.prepare_package(spec, &mut PrintDownload(&spec))?;
        root = &buf;
    }
    info!("system_path: {:?}", root, );
    info!("id path: {:?}", id.vpath()); 
    // Join the path to the root. If it tries to escape, deny
    // access. Note: It can still escape via symlinks.
    id.vpath().resolve(root).ok_or(FileError::AccessDenied)
}

/// Reads a file from a `FileId`.
///
/// If the ID represents stdin it will read from standard input,
/// otherwise it gets the file path of the ID and reads the file from disk.
fn read(id: FileId, project_root: &Path, package_storage: &PackageStorage) -> FileResult<Vec<u8>> {
    info!("read file: {}", project_root.display());
    read_from_disk(&system_path(project_root, id, package_storage)?)
    
}

/// Read a file from disk.
fn read_from_disk(path: &Path) -> FileResult<Vec<u8>> {
    info!("reading file: {}", path.display());
    let f = |e| FileError::from_io(e, path);
    if fs::metadata(path).map_err(f)?.is_dir() {
        Err(FileError::IsDirectory)
    } else {
        debug!("reading file: {}", path.display());
        fs::read(path).map_err(|e| {
            eprintln!("Error reading from disk: {}", e); // Print the error
            f(e) // Call the error handling function f
        })
    }
}

/// Read from stdin.
fn read_from_stdin() -> FileResult<Vec<u8>> {
    let mut buf = Vec::new();
    let result = io::stdin().read_to_end(&mut buf);
    match result {
        Ok(_) => (),
        Err(err) if err.kind() == io::ErrorKind::BrokenPipe => (),
        Err(err) => return Err(FileError::from_io(err, Path::new("<stdin>"))),
    }
    Ok(buf)
}

impl<T: Clone> SlotCell<T> {
    /// Creates a new, empty cell.
    fn new() -> Self {
        Self {
            data: None,
            fingerprint: 0,
            accessed: false,
        }
    }

    /// Whether the cell was accessed in the ongoing compilation.
    fn accessed(&self) -> bool {
        self.accessed
    }

    /// Marks the cell as not yet accessed in preparation of the next
    /// compilation.
    fn reset(&mut self) {
        self.accessed = false;
    }

    /// Gets the contents of the cell or initialize them.
    fn get_or_init(
        &mut self,
        load: impl FnOnce() -> FileResult<Vec<u8>>,
        f: impl FnOnce(Vec<u8>, Option<T>) -> FileResult<T>,
    ) -> FileResult<T> {
        // If we accessed the file already in this compilation, retrieve it.
        if mem::replace(&mut self.accessed, true) {
            if let Some(data) = &self.data {
                return data.clone();
            }
        }

        // Read and hash the file.
        let result = timed!("loading file", load());
        let fingerprint = timed!("hashing file", typst::utils::hash128(&result));

        // If the file contents didn't change, yield the old processed data.
        if mem::replace(&mut self.fingerprint, fingerprint) == fingerprint {
            if let Some(data) = &self.data {
                return data.clone();
            }
        }

        let prev = self.data.take().and_then(Result::ok);
        let value = result.and_then(|data| f(data, prev));
        self.data = Some(value.clone());

        value
    }
}

pub struct ProjectWorld {
    /// The working directory.
    workdir: Option<PathBuf>,
    /// The root relative to which absolute paths are resolved.
    root: PathBuf,
    /// The input path.
    main: FileId,
    /// Typst's standard library.
    library: LazyHash<Library>,
    /// Metadata about discovered fonts.
    book: LazyHash<FontBook>,
    /// Locations of and storage for lazily loaded fonts.
    fonts: Vec<FontSlot>,
    /// Maps file ids to source files and buffers.
    slots: Mutex<HashMap<FileId, FileSlot>>,
    /// Holds information about where packages are stored.
    package_storage: PackageStorage,
    /// The current datetime if requested. This is stored here to ensure it is
    /// always the same within one compilation.
    /// Reset between compilations if not [`Now::Fixed`].
    now: Now,
    /// The export cache, used for caching output files in `typst watch`
    /// sessions.
    export_cache: ExportCache,
}

impl ProjectWorld {
    pub fn new(root: PathBuf, config: ProjectConfig) -> Result<Self, WorldCreationError> {
        let main_path = VirtualPath::within_root(&config.main.expect("input is required"), &root)
            .ok_or(WorldCreationError::InputOutsideRoot)?;
         info!("main_path: {:?} root: {:?} ", main_path, root);
        let main: FileId = FileId::new(None, main_path);

        let library: Library = { Library::builder().build() };

        let now = Now::System(OnceLock::new());
        let fonts = Fonts::searcher().search();

        Ok(Self {
            workdir: Some(root.clone()),
            root,
            main,
            library: LazyHash::new(library),
            book: LazyHash::new(fonts.book),
            fonts: fonts.fonts,
            slots: Mutex::new(HashMap::new()),
            package_storage: package::storage(
                config.package_path,
                config.package_cache_path,
                config.cert,
            ),
            now,
            export_cache: ExportCache::new(),
        })
    }
    pub fn slot_update<P: AsRef<Path>>(
        &mut self,
        path: P,
        content: Option<String>,
    ) -> FileResult<FileId> {
        let vpath = VirtualPath::new(path);
        info!("slot update fn vpath: {:?}", vpath);
        let id = FileId::new(None, vpath.clone());
        self.slot(id, |slot| {
            
            if let Some(res) = &mut slot.source.data {
                info!("res: {:?} vpath: {:?} content: {:?} ", res, vpath, content);
                match self.take_or_read(&vpath, content.clone()) {
                    Ok(content) => match res {
                        Ok(src) => {
                            src.replace(&content);
                        }
                        Err(_) => {
                            *res = Ok(Source::new(id, content));
                        }
                    },
                    Err(e) => {
                        // nothing todo
                    }
                }
            }
            // Write content to slot file
            if let Some(res) = &mut slot.file.data {
                info!("res: {:?} vpath: {:?} content: {:?} ", res, vpath, content);
                match self.take_or_read_bytes(&vpath, content.clone()) {
                    Ok(bytes) => match res {
                        Ok(b) => {
                            *b = bytes;
                        }
                        Err(_) => {
                            *res = Ok(bytes);
                        }
                    },
                    Err(e) => {
                        // nothing todo
                    }
                }
            };
        });

        Ok(id)
    }

    pub fn set_main(&mut self, id: FileId) {
        self.main = id.clone();
    }

    pub fn set_main_path(&mut self, main: VirtualPath) {
        self.set_main(FileId::new(None, main))
    }

    pub fn is_main_set(&self) -> bool {
        // TODO: Check if the file exists
        true
    }

    

    /// Access the canonical slot for the given file id.
    fn slot<F, T>(&self, id: FileId, f: F) -> T
    where
        F: FnOnce(&mut FileSlot) -> T,
    {
        let mut map = self.slots.lock();
        f(map.entry(id).or_insert_with(|| FileSlot::new(id)))
    }

    fn take_or_read(&self, vpath: &VirtualPath, content: Option<String>) -> FileResult<String> {
        if let Some(content) = content {
            return Ok(content);
        }

        let path = vpath.resolve(&self.root).ok_or(FileError::AccessDenied)?;
        fs::read_to_string(&path).map_err(|e| FileError::from_io(e, &path))
    }

    fn take_or_read_bytes(
        &self,
        vpath: &VirtualPath,
        content: Option<String>,
    ) -> FileResult<Bytes> {
        if let Some(content) = content {
            return Ok(Bytes::from(content.into_bytes()));
        }

        let path = vpath.resolve(&self.root).ok_or(FileError::AccessDenied)?;
        fs::read(&path)
            .map_err(|e| FileError::from_io(e, &path))
            .map(Bytes::from)
    }

    fn prepare_package(spec: &PackageSpec) -> PackageResult<PathBuf> {
        let subdir = format!(
            "typst/packages/{}/{}/{}",
            spec.namespace, spec.name, spec.version
        );

        if let Some(data_dir) = dirs::data_dir() {
            let dir = data_dir.join(&subdir);
            info!("----package load_from_path: {:?}", &dir);
            if dir.exists() {
                return Ok(dir);
            }
        }

        if let Some(cache_dir) = dirs::cache_dir() {
            let dir = cache_dir.join(&subdir);
            if dir.exists() {
                return Ok(dir);
            }
            // Download from network if it doesn't exist yet.
            if spec.namespace == "preview" {
                download_package(spec, &dir)?;
                if dir.exists() {
                    return Ok(dir);
                }
            }
        }

        Err(PackageError::NotFound(spec.clone()))
    }
}

impl World for ProjectWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.slot(id, |slot| slot.source(&self.root, &self.package_storage))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.slot(id, |slot| slot.file(&self.root, &self.package_storage))
    }

    fn font(&self, id: usize) -> Option<Font> {
        self.fonts[id].get()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let now = match &self.now {
            Now::Fixed(time) => time,
            Now::System(time) => time.get_or_init(Utc::now),
        };

        // The time with the specified UTC offset, or within the local time zone.
        let with_offset = match offset {
            None => now.with_timezone(&Local).fixed_offset(),
            Some(hours) => {
                let seconds = i32::try_from(hours).ok()?.checked_mul(3600)?;
                now.with_timezone(&FixedOffset::east_opt(seconds)?)
            }
        };

        Datetime::from_ymd(
            with_offset.year(),
            with_offset.month().try_into().ok()?,
            with_offset.day().try_into().ok()?,
        )
    }
    fn packages(&self) -> &[(PackageSpec, Option<typst::diag::EcoString>)] {
        &[]
    }
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
