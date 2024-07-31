use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub name: String,
    pub dtype: String,
    // You can supply metadata as json to a column for user driven features.
    pub metadata: Option<Value>,
}