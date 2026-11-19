use crate::{
    PlayerId,
    client::Client,
    error::Result,
    models::player::{
        equipment::{EquipmentWire, EquippedItem},
        info::{Info, InfoWire},
        me::Me,
        skills::Skill,
    },
};

pub struct PlayerApi<'a> {
    pub(crate) client: &'a Client,
}

impl PlayerApi<'_> {
    pub async fn info(&self, id: impl Into<PlayerId>) -> Result<Info> {
        let id = id.into();
        let wire: InfoWire = self.client.post(&format!("v1/player/info/{id}")).await?;
        Ok(wire.into())
    }

    pub async fn me(&self) -> Result<Me> {
        self.client.post("v2/player/me").await
    }

    pub async fn equipment(&self, id: impl Into<PlayerId>) -> Result<Vec<EquippedItem>> {
        let id: PlayerId = id.into();
        let wire: EquipmentWire = self
            .client
            .post(&format!("v1/player/equipment/{id}"))
            .await?;
        Ok(wire.into())
    }

    pub async fn skills(&self, id: impl Into<PlayerId>) -> Result<Vec<Skill>> {
        let id: PlayerId = id.into();
        self.client.post(&format!("v1/player/skills/{id}")).await
    }
}
