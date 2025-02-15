use crate::aof::delete_aof_file;
use crate::errors::SiderError;
use crate::types::Db;

pub async fn flush_db(db: &Db) -> Result<String, SiderError> {
    db.write().await.clear();
    delete_aof_file().await;
    Ok("\"Ok\"\n".to_string())
}
