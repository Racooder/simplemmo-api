use std::time::Duration;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("no API key configured")]
    MissingApiKey,

    #[error("invalid URL: {0}")]
    Url(#[from] url::ParseError),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API returned status {status}: {body}")]
    Api {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("rate limited (retry after {retry_after:?})")]
    RateLimited { retry_after: Option<Duration> },

    #[error("failed to decode response: {source}")]
    Decode {
        #[source]
        source: serde_json::Error,
        body: String,
    },
}
