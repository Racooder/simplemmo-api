use crate::{
    Client, GuildId, PlayerId, Result,
    models::{
        guild::{
            info::Info, list::GuildSummary, member_contribution::MemberContribution,
            members::GuildMember,
        },
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
        let id: GuildId = id.into();
        self.client.post(&format!("v1/guilds/info/{id}")).await
    }

    pub async fn members(&self, id: impl Into<GuildId>) -> Result<Vec<GuildMember>> {
        let id: GuildId = id.into();
        self.client.post(&format!("v1/guilds/members/{id}")).await
    }

    pub async fn member_contribution(
        &self,
        guild_id: impl Into<GuildId>,
        player_id: impl Into<PlayerId>,
    ) -> Result<MemberContribution> {
        let guild_id: GuildId = guild_id.into();
        let player_id: PlayerId = player_id.into();
        self.client
            .post(&format!(
                "v1/guilds/members/{guild_id}/contribution/{player_id}"
            ))
            .await
    }
}
