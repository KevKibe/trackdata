use std::fmt;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
    pub id: String,
    pub parent_ids: Vec<String>,
    pub message: String,
    pub author: String,
    pub email: String,
    pub root_hash: Option<String>, // Option for now to facilitate migration from older stored commits
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
}

// impl From<Commit> for WorkspaceCommit {
//     fn from(val: Commit) -> Self {
//         WorkspaceCommit {
//             id: val.id,
//             message: val.message,
//             author: val.author,
//             email: val.email,
//             timestamp: val.timestamp,
//         }
//     }
// }

// impl fmt::Display for Commit {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} -> '{}'", self.id, self.message)
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewCommit {
    pub parent_ids: Vec<String>,
    pub message: String,
    pub author: String,
    pub email: String,
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
}