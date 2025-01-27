
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResultType{
    Ok,
    Error
}

impl Default for ResultType {
    fn default() -> Self {
        Self::Ok
    }
}