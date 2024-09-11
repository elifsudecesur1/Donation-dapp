use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use crate::state::{Stake, STAKES};

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> StdResult<Response> {
    let stake = Stake {
        amount: amount.u128(),
        start_time: _env.block.time.seconds(),
        reward_rate: 0.05, // %5 getiri oranı
    };

    STAKES.save(deps.storage, &info.sender, &stake)?;

    Ok(Response::new().add_attribute("action", "stake_funds").add_attribute("amount", amount.to_string()))
}
