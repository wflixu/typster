use ecow::eco_format;
use log::info;
use native_tls::TlsConnector;
use std::collections::VecDeque;
use std::fmt::Display;
use std::fs;
use std::io::{self, ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use typst::diag::{PackageError, PackageResult};
use typst::syntax::package::PackageSpec;
use typst_kit::download::{DownloadState, Downloader, Progress};
use typst_kit::package::PackageStorage;
use typst_utils::format_duration;
use ureq::Response;

const HOST: &str = "https://packages.typst.org";
/// Keep track of this many download speed samples.
const SPEED_SAMPLES: usize = 5;

/// Prints download progress by writing `downloading {0}` followed by repeatedly
/// updating the last terminal line.
pub struct PrintDownload<T>(pub T);

impl<T: Display> Progress for PrintDownload<T> {
    fn print_start(&mut self) {
        println!("downloading");
    }

    fn print_progress(&mut self, state: &DownloadState) {
        let state_str = display_download_progress(state).expect("print progress error");
        println!("{}", state_str);
    }

    fn print_finish(&mut self, state: &DownloadState) {
        let state_str = display_download_progress(state).expect("print progress finish error");
        println!("{}", state_str);
    }
}

/// Compile and format several download statistics and make and attempt at
/// displaying them on standard error.
pub fn display_download_progress(state: &DownloadState) -> io::Result<String> {
    let sum: usize = state.bytes_per_second.iter().sum();
    let len = state.bytes_per_second.len();
    let speed = if len > 0 {
        sum / len
    } else {
        state.content_len.unwrap_or(0)
    };

    let total_downloaded = as_bytes_unit(state.total_downloaded);
    let speed_h = as_throughput_unit(speed);
    let elapsed = Instant::now().saturating_duration_since(state.start_time);
    let res: String;
    match state.content_len {
        Some(content_len) => {
            let percent = (state.total_downloaded as f64 / content_len as f64) * 100.;
            let remaining = content_len - state.total_downloaded;

            let download_size = as_bytes_unit(content_len);
            let eta = Duration::from_secs(if speed == 0 {
                0
            } else {
                (remaining / speed) as u64
            });
            res = format!(
                "{total_downloaded} / {download_size} ({percent:3.0} %) \
                 {speed_h} in {elapsed} ETA: {eta}",
                elapsed = format_duration(elapsed),
                eta = format_duration(eta),
            );
        }
        None => {
            res = format!(
                "{total_downloaded} / {speed_h} in {elapsed}",
                elapsed = format_duration(elapsed),
            )
        }
    };
    Ok(res)
}


/// Format a given size as a unit of time. Setting `include_suffix` to true
/// appends a '/s' (per second) suffix.
fn as_bytes_unit(size: usize) -> String {
    const KI: f64 = 1024.0;
    const MI: f64 = KI * KI;
    const GI: f64 = KI * KI * KI;

    let size = size as f64;

    if size >= GI {
        format!("{:5.1} GiB", size / GI)
    } else if size >= MI {
        format!("{:5.1} MiB", size / MI)
    } else if size >= KI {
        format!("{:5.1} KiB", size / KI)
    } else {
        format!("{size:3} B")
    }
}

fn as_throughput_unit(size: usize) -> String {
    as_bytes_unit(size) + "/s"
}

/// Download from a URL.
#[allow(clippy::result_large_err)]
pub fn download(url: &str) -> Result<ureq::Response, ureq::Error> {
    let mut builder = ureq::AgentBuilder::new();
    let tls = TlsConnector::builder();

    // Set user agent.
    builder = builder.user_agent(concat!("typst/", env!("CARGO_PKG_VERSION")));

    // Get the network proxy config from the environment and apply it.
    if let Some(proxy) = env_proxy::for_url_str(url)
        .to_url()
        .and_then(|url| ureq::Proxy::new(url).ok())
    {
        builder = builder.proxy(proxy);
    }

    // Configure native TLS.
    let connector = tls
        .build()
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    builder = builder.tls_connector(Arc::new(connector));

    builder.build().get(url).call()
}

/// A wrapper around [`ureq::Response`] that reads the response body in chunks
/// over a websocket and displays statistics about its progress.
///
/// Downloads will _never_ fail due to statistics failing to print, print errors
/// are silently ignored.
struct RemoteReader {
    reader: Box<dyn Read + Send + Sync + 'static>,
    content_len: Option<usize>,
    total_downloaded: usize,
    downloaded_this_sec: usize,
    downloaded_last_few_secs: VecDeque<usize>,
    start_time: Instant,
    last_print: Option<Instant>,
}
impl RemoteReader {
    /// Wraps a [`ureq::Response`] and prepares it for downloading.
    ///
    /// The 'Content-Length' header is used as a size hint for read
    /// optimization, if present.
    pub fn from_response(response: Response) -> Self {
        let content_len: Option<usize> = response
            .header("Content-Length")
            .and_then(|header| header.parse().ok());

        Self {
            reader: response.into_reader(),
            content_len,
            total_downloaded: 0,
            downloaded_this_sec: 0,
            downloaded_last_few_secs: VecDeque::with_capacity(SPEED_SAMPLES),
            start_time: Instant::now(),
            last_print: None,
        }
    }

    /// Download the bodies content as raw bytes while attempting to print
    /// download statistics to standard error. Download progress gets displayed
    /// and updated every second.
    ///
    /// These statistics will never prevent a download from completing, errors
    /// are silently ignored.
    pub fn download(mut self) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0; 8192];
        let mut data = match self.content_len {
            Some(content_len) => Vec::with_capacity(content_len),
            None => Vec::with_capacity(8192),
        };

        loop {
            let read = match self.reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => n,
                // If the data is not yet ready but will be available eventually
                // keep trying until we either get an actual error, receive data
                // or an Ok(0).
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };

            data.extend(&buffer[..read]);

            let last_printed = match self.last_print {
                Some(prev) => prev,
                None => {
                    let current_time = Instant::now();
                    self.last_print = Some(current_time);
                    current_time
                }
            };
            let elapsed = Instant::now().saturating_duration_since(last_printed);

            self.total_downloaded += read;
            self.downloaded_this_sec += read;

            if elapsed >= Duration::from_secs(1) {
                if self.downloaded_last_few_secs.len() == SPEED_SAMPLES {
                    self.downloaded_last_few_secs.pop_back();
                }

                self.downloaded_last_few_secs
                    .push_front(self.downloaded_this_sec);
                self.downloaded_this_sec = 0;

                self.last_print = Some(Instant::now());
            }
        }

        Ok(data)
    }
}

/// Download binary data and display its progress.
#[allow(clippy::result_large_err)]
pub fn download_with_progress(url: &str) -> Result<Vec<u8>, ureq::Error> {
    let response = download(url)?;
    Ok(RemoteReader::from_response(response).download()?)
}
/// Download a package over the network.
pub fn download_package(spec: &PackageSpec, package_dir: &Path) -> PackageResult<()> {
    // The `@preview` namespace is the only namespace that supports on-demand
    // fetching.
    assert_eq!(spec.namespace, "preview");

    let url = format!("{HOST}/preview/{}-{}.tar.gz", spec.name, spec.version);

    info!("downloading {}-{}", &spec.name, &spec.version);

    let data = match download_with_progress(&url) {
        Ok(data) => data,
        Err(ureq::Error::Status(404, _)) => return Err(PackageError::NotFound(spec.clone())),
        Err(err) => return Err(PackageError::NetworkFailed(Some(eco_format!("{err}")))),
    };

    let decompressed = flate2::read::GzDecoder::new(data.as_slice());
    tar::Archive::new(decompressed)
        .unpack(package_dir)
        .map_err(|err| {
            fs::remove_dir_all(package_dir).ok();
            PackageError::MalformedArchive(Some(eco_format!("{err}")))
        })
}

/// Returns a new downloader.
pub fn downloader(cert: Option<PathBuf>) -> Downloader {
    let user_agent = concat!("typst/", env!("CARGO_PKG_VERSION"));
    match cert.clone() {
        Some(cert) => Downloader::with_path(user_agent, cert),
        None => Downloader::new(user_agent),
    }
}

/// Returns a new package storage for the given args.
pub fn storage(
    package_path: Option<PathBuf>,
    package_cache_path: Option<PathBuf>,
    cert: Option<PathBuf>,
) -> PackageStorage {
    PackageStorage::new(
        package_cache_path.clone(),
        package_path.clone(),
        downloader(cert),
    )
}
