use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{ids::ItemId, models::player::shared::Stat};

#[derive(Deserialize)]
pub struct EquippedItemWire {
    item_id: ItemId,
    name: String,
    #[serde(rename = "type")]
    item_type: String, // TODO: Enum
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
