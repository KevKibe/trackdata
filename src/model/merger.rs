use crate::model::LocalRepository;
use rocksdb::DB;
pub struct Merger {
    repository: LocalRepository,
    merge_db: DB,
    // files_db: DBWithThreadMode<MultiThreaded>,
}