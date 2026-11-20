use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
#[non_exhaustive]
pub enum SkillType {
    Mining,
    Fishing,
    TreasureHunting,
    Woodcutting,
    LegacyCrafting,
    EventGathering,
    Crafting,
    Other(String),
}

impl From<String> for SkillType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "mining" => Self::Mining,
            "fishing" => Self::Fishing,
            "treasurehunter" => Self::TreasureHunting,
            "woodcutting" => Self::Woodcutting,
            "crafting" => Self::LegacyCrafting,
            "event" => Self::EventGathering,
            "forging" => Self::Crafting,
            _ => Self::Other(s),
        }
    }
}

impl From<SkillType> for String {
    fn from(t: SkillType) -> Self {
        match t {
            SkillType::Mining => "mining".into(),
            SkillType::Fishing => "fishing".into(),
            SkillType::TreasureHunting => "treasurehunter".into(),
            SkillType::Woodcutting => "woodcutting".into(),
            SkillType::LegacyCrafting => "crafting".into(),
            SkillType::EventGathering => "event".into(),
            SkillType::Crafting => "forging".into(),
            SkillType::Other(s) => s,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Skill {
    #[serde(rename = "skill")]
    pub skill_type: SkillType,
    pub level: u64,
    #[serde(rename = "exp")]
    pub experience: u64,
}
