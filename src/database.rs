use crate::aof::get_aof_log_dir;
use crate::process::process_command;
use crate::types::{Db, DbValue};
use tokio::time::{self, Duration};
use tracing::info;

pub async fn delete_expired_keys(db: Db) {
    let mut interval = time::interval(Duration::from_secs(60));

    loop {
        interval.tick().await;
        info!("Deleting expired keys");

        let mut db_write = db.write().await;
        db_write.retain(|_, value| match value {
            DbValue::StringKey(key) => !key.is_expired(),
            DbValue::ListKey(key) => !key.is_expired(),
            DbValue::HashKey(key) => !key.is_expired(),
        });
    }
}

pub async fn restore_from_aof(db: Db) {
    info!("Restoring from AOF file");
    let log_path = get_aof_log_dir();
    let file_path = log_path.join("appendonly.aof");

    if !file_path.exists() {
        return;
    }

    let content = tokio::fs::read_to_string(file_path)
        .await
        .expect("Failed to read AOF file");

    let commands: Vec<&str> = content.split("\n").collect();

    for command in commands {
        if command.is_empty() {
            continue;
        }
        process_command(command.to_string(), &db, false).await;
        info!("Restored: {}", command);
    }
}
