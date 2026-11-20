use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    ids::PlayerId,
    models::{
        de::{bool_from_int, bool_from_nonzero},
        player::shared::{GuildRef, Location, Statistics},
        web::WebPath,
    },
};

#[derive(Deserialize)]
pub struct InfoWire {
    pub id: PlayerId,
    pub name: String,
    pub level: u64,
    #[serde(rename = "avatar")]
    pub avatar_path: String,
    #[serde(rename = "background")]
    pub background_id: u64,
    pub guild: Option<GuildRef>,
    pub motto: String,
    pub profile_number: String,
    #[serde(rename = "exp")]
    pub experience: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_activity: DateTime<Utc>,
    pub gold: u64,

    // Statistics
    pub steps: u64,
    pub npc_kills: u64,
    pub user_kills: u64,
    pub quests_performed: u64,
    #[serde(rename = "quests_complete")]
    pub quests_completed: u64,
    pub tasks_completed: u64,
    pub boss_kills: u64,
    pub market_trades: u64,
    pub reputation: i64,
    pub bounties_completed: u64,
    pub dailies_unlocked: u64,
    pub chests_opened: u64,

    // Stats
    #[serde(rename = "str")]
    pub strength: u64,
    #[serde(rename = "def")]
    pub defence: u64,
    #[serde(rename = "dex")]
    pub dexterity: u64,
    // From my testing bonus stats in v1 seem to be equipment stats in v2
    #[serde(rename = "bonus_dex")]
    pub bonus_dexterity: u64,
    #[serde(rename = "bonus_def")]
    pub bonus_defence: u64,
    #[serde(rename = "bonus_str")]
    pub bonus_strength: u64,

    #[serde(rename = "hp")]
    pub current_health: u64,
    #[serde(rename = "max_hp")]
    pub max_health: u64,
    /// `true` if the player has an active membership.
    /// The API sends an integer; any non-zero value is treated as `true`.
    #[serde(rename = "membership", deserialize_with = "bool_from_nonzero")]
    pub is_member: bool,
    #[serde(rename = "safeMode", deserialize_with = "bool_from_int")]
    pub safe_mode: bool,
    pub creation_date: DateTime<Utc>,
    #[serde(rename = "current_location")]
    pub location: Location,

    #[serde(deserialize_with = "bool_from_int")]
    pub banned: bool,
}

#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct Info {
    pub id: PlayerId,
    pub name: String,
    pub level: u64,
    pub avatar: WebPath,
    pub background: WebPath,
    pub guild: Option<GuildRef>,
    pub motto: String,
    pub profile_number: String,
    pub experience: u64,
    pub last_activity: DateTime<Utc>,
    pub gold: u64,
    pub statistics: Statistics,
    pub stats: InfoStats,
    pub current_health: u64,
    pub max_health: u64,
    pub is_member: bool,
    pub safe_mode: bool,
    pub creation_date: DateTime<Utc>,
    pub location: Location,
    pub banned: bool,
}

impl From<InfoWire> for Info {
    fn from(value: InfoWire) -> Self {
        Self {
            id: value.id,
            name: value.name,
            level: value.level,
            avatar: WebPath::new(value.avatar_path),
            background: WebPath::new(format!("/img/bg/{}.png", value.background_id)),
            guild: value.guild,
            motto: value.motto,
            profile_number: value.profile_number,
            experience: value.experience,
            last_activity: value.last_activity,
            gold: value.gold,
            statistics: Statistics {
                steps: value.steps,
                npc_kills: value.npc_kills,
                user_kills: value.user_kills,
                quests_performed: value.quests_performed,
                quests_completed: value.quests_completed,
                tasks_completed: value.tasks_completed,
                boss_kills: value.boss_kills,
                market_trades: value.market_trades,
                reputation: value.reputation,
                bounties_completed: value.bounties_completed,
                dailies_unlocked: value.dailies_unlocked,
                chests_opened: value.chests_opened,
            },
            stats: InfoStats {
                core: StatBlock {
                    strength: value.strength,
                    defence: value.defence,
                    dexterity: value.dexterity,
                },
                equipment: StatBlock {
                    strength: value.bonus_strength,
                    defence: value.bonus_defence,
                    dexterity: value.bonus_dexterity,
                },
            },
            current_health: value.current_health,
            max_health: value.max_health,
            is_member: value.is_member,
            safe_mode: value.safe_mode,
            creation_date: value.creation_date,
            location: value.location,
            banned: value.banned,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct InfoStats {
    pub core: StatBlock,
    pub equipment: StatBlock,
}

#[derive(Debug, Clone, Serialize)]
pub struct StatBlock {
    pub strength: u64,
    pub defence: u64,
    pub dexterity: u64,
}
