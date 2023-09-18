use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewTimeBlock {
    pub task_id: i64,
    pub start: String,
    pub finish: String,
}