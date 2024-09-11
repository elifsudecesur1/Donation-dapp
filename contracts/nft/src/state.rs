use cosmwasm_std::{Addr};
use serde::{Deserialize, Serialize};
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NftMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub owner: Addr,
}

pub const NFTS: Map<&str, NftMetadata> = Map::new("nfts");
