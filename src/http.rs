use crate::error::Error;
use std::time::Duration;
use ureq::http::Response;
use ureq::{self, Agent, Body};

const TIMEOUT: u64 = 10;

pub fn get(url: &str) -> Result<Response<Body>, Error> {
    let agent = agent();

    match agent.get(url).call() {
        Ok(response) => Ok(response),
        Err(err) => Err(Error::generic(format!("GET {} failed: {}", url, err))),
    }
}

pub fn exists(url: &str) -> bool {
    agent().head(url).call().is_ok()
}

fn agent() -> Agent {
    Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(TIMEOUT)))
        .user_agent(format!("ivm {}", env!("CARGO_PKG_VERSION")))
        .build()
        .into()
}
