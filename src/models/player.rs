use std::collections::HashMap;

use crate::{
    ids::{GuildId, ItemId, LocationId, PlayerId},
    models::de::{bool_from_int, bool_from_nonzero},
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
                    defence: value.defence,
                    dexterity: value.dexterity,
                },
                equipment: InfoStatBlock {
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

#[derive(Deserialize)]
pub struct EquippedItemWire {
    item_id: ItemId,
    name: String,
    #[serde(rename = "type")]
    item_type: String,
    description: String,
    stat1: Option<String>,
    stat1modifier: Option<i64>,
    stat2: Option<String>,
    stat2modifier: Option<i64>,
    stat3: Option<String>,
    stat3modifier: Option<i64>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum EquipmentWire {
    Map(HashMap<String, EquippedItemWire>),
    List(Vec<EquippedItemWire>),
}

impl From<EquipmentWire> for Vec<EquippedItem> {
    fn from(value: EquipmentWire) -> Self {
        let wires: Vec<_> = match value {
            EquipmentWire::Map(m) => m.into_values().collect(),
            EquipmentWire::List(l) => l,
        };
        let mut items: Vec<EquippedItem> = wires.into_iter().map(Into::into).collect();
        items.sort_by_key(|i| i.item_id.0);
        items
    }
}

impl From<EquippedItemWire> for EquippedItem {
    fn from(value: EquippedItemWire) -> Self {
        let modifiers = [
            (value.stat1, value.stat1modifier),
            (value.stat2, value.stat2modifier),
            (value.stat3, value.stat3modifier),
        ]
        .into_iter()
        .filter_map(|(stat, modifier)| {
            let stat = stat.filter(|s| s != "none")?;
            Some(StatModifier {
                stat: Stat::from(stat),
                value: modifier?,
            })
        })
        .collect();

        Self {
            item_id: value.item_id,
            name: value.name,
            item_type: value.item_type,
            description: value.description,
            modifiers,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct EquippedItem {
    pub item_id: ItemId,
    pub name: String,
    pub item_type: String,
    pub description: String,
    pub modifiers: Vec<StatModifier>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StatModifier {
    pub stat: Stat,
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub enum Stat {
    Strength,
    // British
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
    pub defence: u64,
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
    pub defence: u64,
    #[serde(rename = "dex")]
    pub dexterity: u64,
    pub add_crit: u64,
}
