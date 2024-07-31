use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use crate::model::staged_entry::StagedEntryStatus;

// Used for a quick summary of directory
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StagedDirStats {
    pub path: PathBuf,
    pub num_files_staged: usize,
    pub total_files: usize,
    pub status: StagedEntryStatus,
}