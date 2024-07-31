use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::model::field::Field;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schema {
    pub name: Option<String>,
    pub hash: String,
    pub fields: Vec<Field>,
    // Optional string metadata on the schema, to allow for user driven features.
    pub metadata: Option<Value>,
}