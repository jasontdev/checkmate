use serde::{Deserialize, Serialize};
use crate::model::project::Project;
use crate::model::function::Function;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub day_id: i32,
    pub description: String,
    pub project: Project,
    pub task: Function,
}
