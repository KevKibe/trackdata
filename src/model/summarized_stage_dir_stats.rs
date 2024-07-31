use serde::{Deserialize, Serialize};

use crate::model::staged_dir_stats::StagedDirStats;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummarizedStagedDirStats {
    pub num_files_staged: usize,
    pub total_files: usize,
    pub paths: HashMap<PathBuf, Vec<StagedDirStats>>,
}

impl SummarizedStagedDirStats {
    pub fn new() -> SummarizedStagedDirStats {
        SummarizedStagedDirStats {
            num_files_staged: 0,
            total_files: 0,
            paths: HashMap::new(),
        }
    }

}