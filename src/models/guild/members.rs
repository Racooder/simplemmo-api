use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    PlayerId,
    models::{de::bool_from_int, guild::shared::GuildPosition},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildMember {
    pub user_id: PlayerId,
    pub position: GuildPosition,
    pub name: String,
    pub level: u64,
    #[serde(deserialize_with = "bool_from_int")]
    pub safe_mode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub banned: bool,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_activity: DateTime<Utc>,
    pub current_hp: u64,
    pub max_hp: u64,
    #[serde(rename = "warrior", deserialize_with = "bool_from_int")]
    pub is_warrior: bool,
    pub steps: u64,
    pub npc_kills: u64,
    pub user_kills: u64,
}
