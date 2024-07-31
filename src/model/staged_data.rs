
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use crate::model::summarized_stage_dir_stats::SummarizedStagedDirStats;
use crate::model::staged_entry::StagedEntry;
use crate::model::staged_schema::StagedSchema;
use crate::model::merge_conflict::MergeConflict;

#[derive(Debug, Clone)]
pub struct StagedData {
    pub staged_dirs: SummarizedStagedDirStats,
    pub staged_files: HashMap<PathBuf, StagedEntry>, // All the staged entries will be in here
    pub staged_schemas: HashMap<PathBuf, StagedSchema>, // All the staged entrisumes will be in here
    pub untracked_dirs: Vec<(PathBuf, usize)>,
    pub untracked_files: Vec<PathBuf>,
    pub modified_files: Vec<PathBuf>,
    pub moved_files: Vec<(PathBuf, PathBuf, String)>,
    pub removed_files: Vec<PathBuf>,
    pub merge_conflicts: Vec<MergeConflict>,
}


impl StagedData {
    pub fn empty() -> StagedData {
        StagedData {
            staged_dirs: SummarizedStagedDirStats::new(),
            staged_files: HashMap::new(),
            staged_schemas: HashMap::new(),
            untracked_dirs: vec![],
            untracked_files: vec![],
            modified_files: vec![],
            removed_files: vec![],
            moved_files: vec![],
            merge_conflicts: vec![],
        }
    }
}
