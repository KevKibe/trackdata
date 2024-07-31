use rocksdb::{DBWithThreadMode, MultiThreaded};
use crate::model::LocalRepository;
use crate::model::merger::Merger;
use std::path::{Path, PathBuf};
use crate::model::Errors;
use crate::constants;
use crate::model::opts;
pub struct Stager {
    dir_db: DBWithThreadMode<MultiThreaded>,
    schemas_db: DBWithThreadMode<MultiThreaded>,
    pub repository: LocalRepository,
    merger: Option<Merger>,
}

impl Stager {
    pub fn dirs_db_path(path: &Path) -> Result<PathBuf, Errors> {
        let path = crate::hidden_dir(path)
            .join(Path::new(constants::STAGED_DIR))
            .join(constants::DIRS_DIR);

        log::debug!("Stager new dir dir_db_path {:?}", path);
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        Ok(path)
    }

    pub fn schemas_db_path(path: &Path) -> Result<PathBuf, Errors> {
        let path = crate::hidden_dir(path)
            .join(Path::new(constants::STAGED_DIR))
            .join(constants::SCHEMAS_DIR);
        log::debug!("Stager new dir schemas_db_path {:?}", path);
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }
        Ok(path)
    }
    
    pub fn new(repository: &LocalRepository) -> Result<Stager, Errors> {
        let dir_db_path = Stager::dirs_db_path(&repository.path)?;
        let schemas_db_path = Stager::schemas_db_path(&repository.path)?;

        let opts = opts::default();
        Ok(Stager {
            dir_db: DBWithThreadMode::open(&opts, dunce::simplified(&dir_db_path))?,
            schemas_db: DBWithThreadMode::open(&opts, dunce::simplified(&schemas_db_path))?,
            repository: repository.clone(),
            merger: None,
        })
    }
}