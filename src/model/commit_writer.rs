use rocksdb::{DBWithThreadMode, MultiThreaded};
use crate::model::LocalRepository;
use crate::model::opts;
use crate::model::Errors;
use std::path::{Path, PathBuf};
use crate::constants;
use crate::model::commit::{Commit, NewCommit};
use crate::model::staged_data::StagedData;
use time::OffsetDateTime;
use crate::model::user_config::UserConfig;
use crate::model::ref_reader::RefReader;

pub struct CommitWriter {
    pub commits_db: DBWithThreadMode<MultiThreaded>,
    repository: LocalRepository,
}

impl CommitWriter {
    pub fn commit_db_dir(path: &Path) -> PathBuf {
        crate::hidden_dir(path).join(Path::new(constants::COMMITS_DIR))
    }

    pub fn new(repository: &LocalRepository) -> Result<CommitWriter, Errors> {
        let db_path = CommitWriter::commit_db_dir(&repository.path);

        if !db_path.exists() {
            std::fs::create_dir_all(&db_path)?;
        }

        let opts = opts::default();
        Ok(CommitWriter {
            commits_db: DBWithThreadMode::open(&opts, dunce::simplified(&db_path))?,
            repository: repository.clone(),
        })
    }

    fn create_new_commit_data(&self, message: &str) -> Result<NewCommit, Errors> {
        let cfg = UserConfig::get()?;
        let timestamp = OffsetDateTime::now_utc();
        let ref_reader = RefReader::new(&self.repository)?;
        // Commit
        //  - parent_ids (can be empty if root)
        //  - message
        //  - date
        //  - author
        match ref_reader.head_commit_id() {
            Ok(Some(parent_id)) => {
                // We might be in a merge commit, in which case we would have multiple parents
                if self.is_merge_commit() {
                    log::debug!("Create merge commit...");
                    self.create_merge_commit(message)
                } else {
                    // We have one parent
                    log::debug!("Create commit with parent {:?}", parent_id);
                    Ok(NewCommit {
                        parent_ids: vec![parent_id],
                        message: String::from(message),
                        author: cfg.name,
                        email: cfg.email,
                        timestamp,
                    })
                }
            }
            _ => {
                // We are creating initial commit, no parents
                log::debug!("Create initial commit...");
                Ok(NewCommit {
                    parent_ids: vec![],
                    message: String::from(message),
                    author: cfg.name,
                    email: cfg.email,
                    timestamp,
                })
            }
        }
    }

    // Reads commit ids from merge commit files then removes them
    fn create_merge_commit(&self, message: &str) -> Result<NewCommit, Errors> {
        let cfg = UserConfig::get()?;
        let timestamp = OffsetDateTime::now_utc();
        let hidden_dir = crate::hidden_dir(&self.repository.path);
        let merge_head_path = hidden_dir.join(MERGE_HEAD_FILE);
        let orig_head_path = hidden_dir.join(ORIG_HEAD_FILE);

        // Read parent commit ids
        let merge_commit_id = util::fs::read_from_path(&merge_head_path)?;
        let head_commit_id = util::fs::read_from_path(&orig_head_path)?;

        // Cleanup
        util::fs::remove_file(merge_head_path)?;
        util::fs::remove_file(orig_head_path)?;

        Ok(NewCommit {
            parent_ids: vec![merge_commit_id, head_commit_id],
            message: String::from(message),
            author: cfg.name,
            email: cfg.email,
            timestamp,
        })
    }

    fn is_merge_commit(&self) -> bool {
        let hidden_dir = crate::hidden_dir(&self.repository.path);
        let merge_head_path = hidden_dir.join(MERGE_HEAD_FILE);
        merge_head_path.exists()
    }

    pub fn commit(&self, status: &StagedData, message: &str) -> Result<Commit, Errors> {
        // Create a commit object, that either points to parent or not
        // must create this before anything else so that we know if it has parent or not.
        log::debug!("---COMMIT START---"); // for debug logging / timing purposes
        let new_commit = self.create_new_commit_data(message)?;
        log::debug!("Created commit obj {:?}", new_commit);
        let commit = self.commit_from_new(&new_commit, status, &self.repository.path)?;
        log::debug!("COMMIT_COMPLETE {} -> {}", commit.id, commit.message);

        // Mark as synced so we know we don't need to pull versions files again
        index::commit_sync_status::mark_commit_as_synced(&self.repository, &commit)?;

        // User output
        println!("Commit {} done.", commit.id);
        log::debug!("---COMMIT END {} -> '{}'---", commit.id, message); // for debug logging / timing purposes
        Ok(commit)
    }

}