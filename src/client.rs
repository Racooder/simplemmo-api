use reqwest::{StatusCode, Url, header::RETRY_AFTER};
use serde::de::DeserializeOwned;
use std::{sync::Arc, time::Duration};

use crate::{
    api::{guild::GuildApi, player::PlayerApi},
    error::{Error, Result},
};

const DEFAULT_BASE_URL: &str = "https://api.simple-mmo.com/";

#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

struct Inner {
    http: reqwest::Client,
    base_url: Url,
    api_key: String,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn player(&self) -> PlayerApi<'_> {
        PlayerApi { client: self }
    }

    pub fn guild(&self) -> GuildApi<'_> {
        GuildApi { client: self }
    }

    // Later: items(), market(), orphanage(), world_bosses()

    pub(crate) async fn post<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.post_query(path, &[]).await
    }

    pub(crate) async fn post_query<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<T> {
        let url = self.inner.base_url.join(path)?;
        let resp = self
            .inner
            .http
            .post(url)
            .query(query)
            .json(&serde_json::json!({ "api_key": self.inner.api_key }))
            .send()
            .await?;

        let status = resp.status();

        if status == StatusCode::TOO_MANY_REQUESTS {
            let retry_after = resp
                .headers()
                .get(RETRY_AFTER)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .map(Duration::from_secs);
            return Err(Error::RateLimited { retry_after });
        }

        let bytes = resp.bytes().await?;

        if !status.is_success() {
            return Err(Error::Api {
                status,
                body: String::from_utf8_lossy(&bytes).into_owned(),
            });
        }

        serde_json::from_slice(&bytes).map_err(|source| Error::Decode {
            source,
            body: String::from_utf8_lossy(&bytes).into_owned(),
        })
    }
}

#[derive(Default)]
pub struct ClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    user_agent: Option<String>,
    http: Option<reqwest::Client>,
}

impl ClientBuilder {
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    pub fn http_client(mut self, http: reqwest::Client) -> Self {
        self.http = Some(http);
        self
    }

    pub fn build(self) -> Result<Client> {
        let api_key = self.api_key.ok_or(Error::MissingApiKey)?;

        let mut base = self.base_url.unwrap_or_else(|| DEFAULT_BASE_URL.into());
        if !base.ends_with('/') {
            base.push('/');
        }
        let base_url = Url::parse(&base)?;

        let http = match self.http {
            Some(h) => h,
            None => reqwest::Client::builder()
                .user_agent(self.user_agent.unwrap_or_else(|| {
                    concat!("simplemmo-client/", env!("CARGO_PKG_VERSION")).into()
                }))
                .timeout(Duration::from_secs(30))
                .build()?,
        };

        Ok(Client {
            inner: Arc::new(Inner {
                http,
                base_url,
                api_key,
            }),
        })
    }
}
