use crate::ids::{GuildId, LocationId, PlayerId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PlayerMe {
    pub id: PlayerId,
    pub name: String,
    pub level: u64,
    pub avatar_url: String,
    pub background_url: String,
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
    pub membership: u8, // TODO: Boolean?
    pub safe_mode: bool,
    pub safe_mode_expires_at: Option<DateTime<Utc>>,
    pub creation_date: DateTime<Utc>,
    pub location: Location,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildRef {
    pub id: GuildId,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Location {
    pub id: LocationId,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Currencies {
    pub gold: u64,
    pub diamonds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Statistics {
    pub steps: u64,
    pub npc_kills: u64,
    pub user_kills: u64,
    pub quests_performed: u64,
    pub quests_completed: u64,
    pub tasks_completed: u64,
    pub boss_kills: u64,
    pub market_trades: u64,
    pub reputation: i64, // TODO: Check if negative values are possible
    pub bounties_completed: u64,
    pub dailies_unlocked: u64,
    pub chests_opened: u64,
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
    pub def: u64,
    pub dex: u64,
    pub add_crit: u64,
}
