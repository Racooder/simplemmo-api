use crate::{
    PlayerId,
    client::Client,
    error::Result,
    models::player::{PlayerInfo, PlayerInfoWire, PlayerMe},
};

pub struct PlayerApi<'a> {
    pub(crate) client: &'a Client,
}

impl PlayerApi<'_> {
    pub async fn info(&self, id: impl Into<PlayerId>) -> Result<PlayerInfo> {
        let id = id.into();
        let wire: PlayerInfoWire = self.client.post(&format!("v1/player/info/{id}")).await?;
        Ok(wire.into())
    }

    pub async fn me(&self) -> Result<PlayerMe> {
        self.client.post("v2/player/me").await
    }
}
