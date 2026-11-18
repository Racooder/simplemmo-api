use crate::{
    ids::{GuildId, LocationId, PlayerId},
    models::de::bool_from_int,
    models::de::bool_from_nonzero,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PlayerInfoWire {
    pub id: PlayerId,
    pub name: String,
    pub level: u64,
    #[serde(rename = "avatar")]
    pub avatar_url: String,
    // Maps to background_url with /img/bg/<background_id>.png
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
    pub defense: u64,
    #[serde(rename = "dex")]
    pub dexterity: u64,
    // From my testing bonus stats in v1 seem to be equipment stats in v2
    #[serde(rename = "bonus_dex")]
    pub bonus_dexterity: u64,
    #[serde(rename = "bonus_def")]
    pub bonus_defense: u64,
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
pub struct PlayerInfo {
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

impl From<PlayerInfoWire> for PlayerInfo {
    fn from(value: PlayerInfoWire) -> Self {
        let background_id = value.background_id;
        Self {
            id: value.id,
            name: value.name,
            level: value.level,
            avatar_url: value.avatar_url,
            background_url: format!("/img/bg/{background_id}.png"),
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
                core: InfoStatBlock {
                    strength: value.strength,
                    defense: value.defense,
                    dexterity: value.dexterity,
                },
                equipment: InfoStatBlock {
                    strength: value.bonus_strength,
                    defense: value.bonus_defense,
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

#[derive(Debug, Clone, Serialize)]
pub struct InfoStats {
    pub core: InfoStatBlock,
    pub equipment: InfoStatBlock,
}

#[derive(Debug, Clone, Serialize)]
pub struct InfoStatBlock {
    pub strength: u64,
    pub defense: u64,
    pub dexterity: u64,
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
    pub defense: u64,
    #[serde(rename = "dex")]
    pub dexterity: u64,
    pub add_crit: u64,
}
