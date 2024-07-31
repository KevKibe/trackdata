mod model;
mod constants;

use std::path::Path;
use crate::model::{LocalRepository,Errors};
use crate::model::commit::Commit;
use crate::model::staged_data::StagedData;
use crate::model::commit_writer::CommitWriter;
use crate::model::stager::Stager;
use std::path::PathBuf;



pub fn init(path: &Path) -> Result<LocalRepository, Errors> {
    let hidden_dir = hidden_dir(path);
    if hidden_dir.exists() {
        let err = format!("Repository already exists: {path:?}");
        return Err(Errors::RepoAlreadyExists(err));
    }

    match p_init(path) {
        Ok(result) => Ok(result),
        Err(error) => {
            remove_dir_all(hidden_dir).map_err(Errors::from)?;
            Err(Errors::Other(error.to_string()))
        }
    }
}

fn p_init(path: &Path) -> Result<LocalRepository, Errors> {
    let hidden_dir = hidden_dir(path);

    std::fs::create_dir_all(hidden_dir)?;
    let config_path = config_filepath(path);
    let repo = LocalRepository::new(path)?;
    repo.save(&config_path)?;

    commit_with_no_files(&repo,constants::INITIAL_COMMIT_MSG)?;

    Ok(repo)
}


pub fn hidden_dir(repo_path: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(repo_path.as_ref()).join(Path::new(constants::TRACK_DATA_DIR))
}

pub fn config_filepath(repo_path: &Path) -> PathBuf {
    hidden_dir(repo_path).join(constants::REPO_CONFIG_FILENAME)
}

pub fn remove_dir_all(src: impl AsRef<Path>) -> Result<(), Errors> {
    let src = src.as_ref();
    match std::fs::remove_dir_all(src) {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!("remove_dir_all {:?} {}", src, err);
            Err(Errors::FileError(src.to_string_lossy().into_owned(), err))
        }
    }
}


pub fn commit_with_no_files(repo: &LocalRepository, message: &str) -> Result<Commit, Errors> {
    let status = StagedData::empty();
    let commit = commit(repo, &status, message)?;
    println!("Initial commit {}", commit.id);
    Ok(commit)
}

pub fn commit(
    repo: &LocalRepository,
    status: &StagedData,
    message: &str,
) -> Result<Commit, Errors> {
    let stager = Stager::new(repo)?;
    let commit_writer = CommitWriter::new(repo)?;
    let commit = commit_writer.commit(status, message)?;
    stager.unstage()?;
    Ok(commit)
}


fn main() {
    let repo_path = Path::new("test-repo");
    
    match init(repo_path) {
        Ok(repo) => println!("Repository created successfully: {:?}", repo),
        Err(e) => println!("Failed to create repository: {}", e),
    }
}




