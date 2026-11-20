use serde::{Deserialize, Serialize};
use url::Url;

const WEB_BASE: &str = "https://web.simple-mmo.com/";

/// Path of a file hosted on the SimpleMMO website, e.g. `/img/bg/207.png`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WebPath(String);

impl WebPath {
    pub(crate) fn new(path: impl Into<String>) -> Self {
        Self(path.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn url(&self) -> Option<Url> {
        Url::parse(WEB_BASE).ok()?.join(&self.0).ok()
    }
}
