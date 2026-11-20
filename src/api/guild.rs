use crate::{
    Client, GuildId, Result,
    models::{
        guild::{info::Info, list::GuildSummary},
        page::{Page, PageWire},
    },
};

pub struct GuildApi<'a> {
    pub(crate) client: &'a Client,
}

impl GuildApi<'_> {
    /// Lists guilds, 20 per page. `page` starts at 1.
    pub async fn list(&self, page: u32) -> Result<Page<GuildSummary>> {
        let wire: PageWire<GuildSummary> = self
            .client
            .post_query("v1/guilds/all", &[("page", page.to_string())])
            .await?;
        Ok(wire.into())
    }

    pub async fn info(&self, id: impl Into<GuildId>) -> Result<Info> {
        let id = id.into();
        self.client.post(&format!("v1/guilds/info/{id}")).await
    }
}
