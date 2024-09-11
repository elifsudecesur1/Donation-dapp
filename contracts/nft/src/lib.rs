use cosmwasm_std::{Binary, DepsMut, Env, MessageInfo, Response, StdResult};
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{NftMetadata, NFTS};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::MintNft { token_id, owner, metadata } => mint_nft(deps, info, token_id, owner, metadata),
        _ => unimplemented!(),
    }
}

fn mint_nft(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    owner: String,
    metadata: NftMetadata,
) -> StdResult<Response> {
    NFTS.save(deps.storage, &token_id, &metadata)?;

    Ok(Response::new().add_attribute("action", "mint_nft").add_attribute("owner", owner))
}
