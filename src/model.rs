mod remote_config;
pub mod commit;
pub mod staged_data;
mod summarized_stage_dir_stats;
mod staged_dir_stats;
mod staged_entry;
mod staged_schema;
pub mod commit_writer;
mod merger;
pub mod stager;
mod merge_conflict;
mod commit_entry;
mod schema;
mod field;
mod opts;
mod user_config;
mod ref_reader;

use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};
use crate::model::remote_config::{RemoteConfig, Remote};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalRepository {
    pub path: PathBuf,
    // Optional remotes to sync the data to
    remote_name: Option<String>, // name of the current remote ("origin" by default)
    pub remotes: Vec<Remote>,    // List of possible remotes
}
impl LocalRepository {
    // Create a brand new repository with new ID
    pub fn new(path: &Path) -> Result<LocalRepository, Errors> {
        Ok(LocalRepository {
            path: path.to_path_buf(),
            remotes: vec![],
            remote_name: None,
        })
    }

    pub fn save(&self, path: &Path) -> Result<(), Errors> {
        let cfg = RemoteConfig {
            remote_name: self.remote_name.clone(),
            remotes: self.remotes.clone(),
        };
        // let toml = toml::to_string(&cfg)?;
        // util::fs::write_to_path(path, toml)?;
        Ok(())
    }

}

#[derive(Debug)]
pub enum Errors {
    RepoAlreadyExists(String),
    IoError(std::io::Error),
    Other(String),
    FileError(String, std::io::Error),
    EmailAndNameNotFound(String)

}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::RepoAlreadyExists(msg) => write!(f, "Repository already exists: {}", msg),
            Errors::IoError(err) => write!(f, "IO error: {}", err),
            Errors::Other(msg) => write!(f, "Error: {}", msg),
            Errors::FileError(path, err) => write!(f, "File error at {}: {}", path, err),
            Errors::EmailAndNameNotFound(msg) => write!(f, "Email and NAme not found : {}", msg),
        }
    }
}

impl std::error::Error for Errors {}

impl From<std::io::Error> for Errors {
    fn from(error: std::io::Error) -> Self {
        Errors::IoError(error)
    }
}


pub trait ContentHashable {
    fn content_hash(&self) -> String;
}