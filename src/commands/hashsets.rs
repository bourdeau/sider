use crate::types::Command;
use crate::types::Db;

pub async fn hset(db: &Db, command: Command) -> String {
    "ok".to_string()
}
