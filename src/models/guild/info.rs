use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    GuildId, PlayerId,
    models::{
        de::{bool_from_int, icon_path},
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
    pub members: Vec<GuildMember>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildMember {
    #[serde(rename = "userid")]
    pub player: PlayerId,
    pub position: GuildPosition,
    #[serde(rename = "warrior", deserialize_with = "bool_from_int")]
    pub is_warrior: bool,
    #[serde(rename = "created_at")]
    pub joined_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
#[non_exhaustive]
pub enum GuildPosition {
    Leader,
    CoLeader,
    Officer,
    Member,
    Other(String),
}

impl From<String> for GuildPosition {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Leader" => Self::Leader,
            "Co Leader" => Self::CoLeader,
            "Officer" => Self::Officer,
            "Member" => Self::Member,
            _ => Self::Other(s),
        }
    }
}

impl From<GuildPosition> for String {
    fn from(p: GuildPosition) -> Self {
        match p {
            GuildPosition::Leader => "Leader".into(),
            GuildPosition::CoLeader => "Co Leader".into(),
            GuildPosition::Officer => "Officer".into(),
            GuildPosition::Member => "Member".into(),
            GuildPosition::Other(s) => s,
        }
    }
}
