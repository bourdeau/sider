use crate::aof::delete_aof_file;
use crate::errors::SiderError;
use crate::response::SiderResponse;
use crate::types::Db;

pub async fn flush_db(db: &Db) -> Result<SiderResponse, SiderError> {
    db.write().await.clear();
    delete_aof_file().await;
    Ok(SiderResponse::Ok)
}
