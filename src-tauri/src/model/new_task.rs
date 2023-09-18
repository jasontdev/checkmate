use serde::{Deserialize, Serialize};
use crate::model::TimeBlock;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewTask {
    pub description: String,
    pub time_blocks: Vec<TimeBlock>,
}
