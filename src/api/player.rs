use crate::{client::Client, error::Result, models::player::PlayerMe};

pub struct PlayerApi<'a> {
    pub(crate) client: &'a Client,
}

impl PlayerApi<'_> {
    pub async fn me(&self) -> Result<PlayerMe> {
        self.client.post("v2/player/me").await
    }
}
