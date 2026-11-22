use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    GuildId, PlayerId,
    models::{
        de::{bool_from_int, icon_path},
        guild::shared::GuildPosition,
        web::WebPath,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Info {
    pub id: GuildId,
    pub name: String,
    pub tag: String,
    pub owner: PlayerId,
    #[serde(rename = "exp")]
    pub experience: u64,
    #[serde(rename = "current_season_exp")]
    pub current_season_experience: u64,
    #[serde(deserialize_with = "bool_from_int")]
    pub passive: bool,
    #[serde(deserialize_with = "icon_path")]
    pub icon: WebPath,
    #[serde(rename = "legacy_exp")]
    pub legacy_experience: u64,
    pub member_count: u64,
    pub eligible_for_guild_war: bool,
    pub members: Vec<GuildMemberSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildMemberSummary {
    #[serde(rename = "userid")]
    pub player: PlayerId,
    pub position: GuildPosition,
    #[serde(rename = "warrior", deserialize_with = "bool_from_int")]
    pub is_warrior: bool,
    #[serde(rename = "created_at")]
    pub joined_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
