use serde::{Deserialize, Serialize};

use crate::{GuildId, ids::LocationId};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub enum Stat {
    Strength,
    // British naming because the API uses british names
    Defence,
    Dexterity,
    Other(String),
}

impl From<String> for Stat {
    fn from(value: String) -> Self {
        match value.as_str() {
            "str" => Stat::Strength,
            "def" => Stat::Defence,
            "dex" => Stat::Dexterity,
            _ => Stat::Other(value),
        }
    }
}
