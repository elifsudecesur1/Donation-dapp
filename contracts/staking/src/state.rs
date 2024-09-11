use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Stake {
    pub amount: u128,
    pub start_time: u64,
    pub reward_rate: f64,
}

pub const STAKES: Map<&Addr, Stake> = Map::new("stakes");
