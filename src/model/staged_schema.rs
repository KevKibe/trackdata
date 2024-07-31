use crate::model::ContentHashable;
use crate::model::staged_entry::StagedEntryStatus;
use serde::{Deserialize, Serialize};
use crate::model::schema::Schema;
// use super::Schema;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub enum StagedSchemaStatus {
    Added,
    Modified,
    Removed,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StagedSchema {
    pub schema: Schema,
    pub status: StagedEntryStatus,
}

impl ContentHashable for StagedSchema {
    fn content_hash(&self) -> String {
        self.schema.hash.clone()
    }
}