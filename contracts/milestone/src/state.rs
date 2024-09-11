use serde::{Deserialize, Serialize};
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Milestone {
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Project {
    pub milestones: Vec<Milestone>,
}

pub const PROJECTS: Map<u64, Project> = Map::new("projects");
