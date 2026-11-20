use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    PlayerId,
    models::{
        de::bool_from_nonzero,
        player::shared::{GuildRef, Location, Statistics},
        web::WebPath,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Me {
    pub id: PlayerId,
    pub name: String,
    pub level: u64,
    #[serde(rename = "avatar_url")]
    pub avatar: WebPath,
    #[serde(rename = "background_url")]
    pub background: WebPath,
    pub guild: Option<GuildRef>,
    pub motto: String,
    pub profile_number: String,
    pub experience: u64,
    pub last_activity: DateTime<Utc>,
    pub currencies: Currencies,
    pub statistics: Statistics,
    pub stats: Stats,
    pub available_stat_points: u64,
    pub current_health: u64,
    pub max_health: u64,
    pub current_energy: u64,
    pub max_energy: u64,
    pub current_quest_points: u64,
    pub max_quest_points: u64,
    /// `true` if the player has an active membership.
    /// The API sends an integer; any non-zero value is treated as `true`.
    #[serde(rename = "membership", deserialize_with = "bool_from_nonzero")]
    pub is_member: bool,
    pub safe_mode: bool,
    pub safe_mode_expires_at: Option<DateTime<Utc>>,
    pub creation_date: DateTime<Utc>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Currencies {
    pub gold: u64,
    pub diamonds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub core: StatBlock,
    pub equipment: StatBlock,
    pub bonus: BonusStats,
    pub total: StatBlock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BonusStats {
    pub fixed: StatBlock,
    pub percentage: StatBlock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatBlock {
    #[serde(rename = "str")]
    pub strength: u64,
    #[serde(rename = "def")]
    pub defence: u64,
    #[serde(rename = "dex")]
    pub dexterity: u64,
    pub add_crit: u64,
}
