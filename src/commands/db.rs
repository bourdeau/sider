use crate::aof::delete_aof_file;
use crate::types::Db;

pub async fn flush_db(db: &Db) -> String {
    db.write().await.clear();
    // delete aof file
    delete_aof_file().await;
    "OK\n".to_string()
}
