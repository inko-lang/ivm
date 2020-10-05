use crate::error::Error;
use ureq::{self, Request, Response};

const TIMEOUT: u64 = 10 * 1_000;

pub fn get(url: &str) -> Result<Response, Error> {
    let response = prepare(ureq::get(url)).call();

    if response.ok() {
        Ok(response)
    } else {
        Err(Error::generic(format!(
            "GET {} failed: HTTP {} {}",
            url,
            response.status(),
            response.status_text()
        )))
    }
}

pub fn exists(url: &str) -> bool {
    prepare(ureq::head(url)).call().ok()
}

fn prepare(mut request: Request) -> Request {
    request
        .timeout_connect(TIMEOUT)
        .timeout_read(TIMEOUT)
        .set("User-Agent", &format!("ivm {}", env!("CARGO_PKG_VERSION")));

    request
}
