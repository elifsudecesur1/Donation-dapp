use serde::{Deserialize, Serialize};
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Proposal {
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
}

pub const PROPOSALS: Map<u64, Proposal> = Map::new("proposals");
