use serde::{Deserialize, Serialize};
use crate::constants;
use std::path::{Path, PathBuf};
use crate::model::Errors;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserConfig {
    pub name: String,
    pub email: String,
}

impl UserConfig {
    pub fn get() -> Result<UserConfig, Errors> {
        let config_dir = crate::config_dir()?;
        let mut config_file = config_dir.join(Path::new(constants::USER_CONFIG_FILENAME));
        if std::env::var("TEST").is_ok() {
            config_file = PathBuf::from("data/test/config/user_config.toml");
        }
        log::debug!("looking for config file in...{:?}", config_file);
        if config_file.exists() {
            Ok(UserConfig::new(&config_file))
        } else {
            log::debug!(
                "unable to find config file at {:?}. Current working directory is {:?}",
                config_file,
                std::env::current_dir().unwrap()
            );
            
            Err(Errors::EmailAndNameNotFound())
        }
    }

}