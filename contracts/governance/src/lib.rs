use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use crate::state::{Proposal, PROPOSALS};

pub fn execute_vote(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    proposal_id: u64,
    approve: bool,
) -> StdResult<Response> {
    let mut proposal = PROPOSALS.load(deps.storage, proposal_id)?;

    if approve {
        proposal.votes_for += 1;
    } else {
        proposal.votes_against += 1;
    }

    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;

    Ok(Response::new().add_attribute("action", "vote"))
}
