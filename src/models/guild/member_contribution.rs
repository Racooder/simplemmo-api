use serde::{Deserialize, Serialize};

use crate::PlayerId;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MemberContribution {
    pub user_id: PlayerId,
    pub gold_deposited: u64,
    pub power_points_deposited: u64,
    pub pve_kills: u64,
    #[serde(rename = "pve_exp")]
    pub pve_experience: u64,
    pub pvp_kills: u64,
    #[serde(rename = "pvp_exp")]
    pub pvp_experience: u64,
    pub steps_taken: u64,
    pub steps_experience: u64,
    pub nodes_gathered: u64,
    pub nodes_experience: u64,
    pub tax_contribution: TaxContribution,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TaxContribution {
    pub guild_bank: u64,
    pub sanctuary: u64,
}
