use ecow::eco_format;
use log::info;
use native_tls::TlsConnector;
use std::collections::VecDeque;
use std::fs;
use std::io::{self, ErrorKind, Read};
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
use typst::diag::{PackageError, PackageResult};
use typst::syntax::package::PackageSpec;
use ureq::Response;

const HOST: &str = "https://packages.typst.org";
/// Keep track of this many download speed samples.
const SPEED_SAMPLES: usize = 5;

/// Download from a URL.
#[allow(clippy::result_large_err)]
pub fn download(url: &str) -> Result<ureq::Response, ureq::Error> {
    let mut builder = ureq::AgentBuilder::new();
    let mut tls = TlsConnector::builder();

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
