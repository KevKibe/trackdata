use rocksdb::DB;
use std::path::PathBuf;
use crate::model::LocalRepository;
use crate::constants;
use crate::model::Errors;
use crate::model::opts;

pub struct RefReader {
    refs_db: DB,
    head_file: PathBuf,
    repository: LocalRepository,
}

impl RefReader {
    pub fn new(repository: &LocalRepository) -> Result<RefReader, Errors> {
        let refs_dir = crate::hidden_dir(&repository.path).join(constants::REFS_DIR);
        let head_filename = crate::hidden_dir(&repository.path).join(constants::HEAD_FILE);
        let error_if_log_file_exist = false;
        let opts = opts::default();

        if !refs_dir.exists() {
            std::fs::create_dir_all(&refs_dir)?;
            // open it then lose scope to close it
            // so that we can read an empty one if it doesn't exist
            let _db = DB::open(&opts, dunce::simplified(&refs_dir))?;
        }

        Ok(RefReader {
            refs_db: DB::open_for_read_only(
                &opts,
                dunce::simplified(&refs_dir),
                error_if_log_file_exist,
            )?,
            head_file: head_filename,
            repository: repository.clone(),
        })
    }


    pub fn read_head_ref(&self) -> Result<Option<String>, Errors> {
        // Should probably lock before reading...
        // but not a lot of parallel action going on here
        // log::debug!("Looking for HEAD at {:?}", self.head_file);
        if self.head_file.exists() {
            Ok(Some(util::fs::read_from_path(&self.head_file)?))
        } else {
            log::debug!("HEAD not found at {:?}", self.head_file);
            Ok(None)
        }
    }

    pub fn head_commit_id(&self) -> Result<Option<String>, Errors> {
        let head_ref = self.read_head_ref()?;
        // log::debug!("Got HEAD ref {:?}", head_ref);

        if let Some(head_ref) = head_ref {
            if let Some(commit_id) = self.get_commit_id_for_branch(&head_ref)? {
                log::debug!(
                    "RefReader::head_commit_id got commit id {} for branch {}",
                    commit_id,
                    head_ref
                );
                Ok(Some(commit_id))
            } else {
                log::debug!(
                    "RefReader::head_commit_id looking for head_ref {}",
                    head_ref
                );
                let commit_reader = CommitReader::new(&self.repository)?;
                if commit_reader.commit_id_exists(&head_ref) {
                    Ok(Some(head_ref))
                } else {
                    log::debug!("Commit id does not exist {:?}", head_ref);
                    Ok(None)
                }
            }
        } else {
            log::debug!("Head ref is none {:?}", head_ref);
            Ok(None)
        }
    }

}