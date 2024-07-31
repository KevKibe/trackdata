use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CommitEntry {
    pub commit_id: String,
    pub path: PathBuf,
    pub hash: String,
    pub num_bytes: u64,
    pub last_modified_seconds: i64,
    pub last_modified_nanoseconds: u32,
}