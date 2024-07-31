use serde::{Deserialize, Serialize};
use std::path::Path;

// use crate::model::Remote;
// use crate::util;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteConfig {
    pub remote_name: Option<String>, // this is the current remote name
    pub remotes: Vec<Remote>,
}

impl Default for RemoteConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl RemoteConfig {
    pub fn new() -> Self {
        RemoteConfig {
            remote_name: None,
            remotes: Vec::new(),
        }
    }

    // pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Errors> {
    //     let contents = util::fs::read_from_path(&path)?;
    //     let remote_config: RemoteConfig = toml::from_str(&contents)?;
    //     Ok(remote_config)
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Remote {
    pub name: String,
    pub url: String,
}

impl std::fmt::Display for Remote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] '{}'", self.name, self.url)
    }
}

impl std::error::Error for Remote {}