use serde::{Deserialize, Serialize};

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
