use crate::{
    GuildId, PlayerId,
    models::{
        de::{bool_from_nonzero, icon_path},
        web::WebPath,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildSummary {
    pub id: GuildId,
    pub name: String,
    pub tag: String,
    pub owner: PlayerId,
    #[serde(deserialize_with = "bool_from_nonzero")]
    pub passive: bool,
    #[serde(deserialize_with = "icon_path")]
    pub icon: WebPath,
    pub total_experience: u64,
    #[serde(rename = "current_season_exp")]
    pub current_season_experience: u64,
}
