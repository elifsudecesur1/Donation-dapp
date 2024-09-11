use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use crate::state::{Milestone, PROJECTS};

pub fn release_funds_by_milestone(
    deps: DepsMut,
    _env: Env,
    project_id: u64,
    milestone_index: usize,
) -> StdResult<Response> {
    let mut project = PROJECTS.load(deps.storage, project_id)?;

    let milestone = &mut project.milestones[milestone_index];
    if milestone.completed {
        return Err(StdError::generic_err("Milestone already completed"));
    }

    milestone.completed = true;
    PROJECTS.save(deps.storage, project_id, &project)?;

    Ok(Response::new().add_attribute("action", "release_funds"))
}
