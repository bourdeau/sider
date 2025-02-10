use crate::types::{Command, CommandArgs, Db, DbValue};
use dirs::home_dir;
use std::io::Error;
use std::path::PathBuf;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::time::{self, Duration};
use tracing::info;

pub fn get_aof_log_dir() -> PathBuf {
    let home = home_dir().expect("Failed to get home directory");
    home.join(".local/share/sider")
}

pub async fn delete_aof_file() {
    let aof_log_dir = get_aof_log_dir();
    if aof_log_dir.exists() {
        let file_path = aof_log_dir.join("appendonly.aof");
        let _ = fs::remove_file(&file_path).await;
    }
}

pub fn get_aof_file() -> PathBuf {
    let log_path = get_aof_log_dir();
    log_path.join("appendonly.aof")
}

pub async fn write_aof(command: &Command) -> std::io::Result<()> {
    let log_path = get_aof_log_dir();

    if !log_path.exists() {
        fs::create_dir_all(&log_path).await?;
    }

    let keys_value = match &command.args {
        CommandArgs::SingleKey(key) => key.get_name_value_as_string(),
        CommandArgs::MultipleKeys(keys) => keys
            .iter()
            .map(|key| key.get_name_value_as_string())
            .collect::<Vec<_>>()
            .join(" "),
        CommandArgs::KeyWithValues(list_key) => {
            format!("{} {}", list_key.name, list_key.values.join(" "))
        }
    };

    let formatted = format!("{:?} {}\n", command.command_type, keys_value);

    let file_path = log_path.join("appendonly.aof");

    let mut file = File::options()
        .append(true)
        .create(true)
        .open(&file_path)
        .await?;

    file.write_all(formatted.as_bytes()).await?;

    Ok(())
}

async fn dump_db_to_aof(db: &Db) -> Result<(), Error> {
    let db_write = db.write().await;
    let db_dump_aof = get_aof_log_dir().join("db-dump.aof");
    let aof_file = get_aof_file();

    let mut file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&db_dump_aof)
        .await?;

    let mut output = String::new();

    for (key, value) in db_write.iter() {
        match value {
            DbValue::StringKey(k) => {
                if let Some(val) = &k.value {
                    output.push_str(&format!("SET {} {}\n", key, val));
                }
            }

            DbValue::ListKey(l) => {
                let values = l.values.join(" ");
                output.push_str(&format!("LPUSH {} {}\n", key, values));
            }
        }
    }

    file.write_all(output.as_bytes()).await?;

    // Ensure all data is written
    file.flush().await?;

    // Deleting actual aof file
    delete_aof_file().await;

    // Replacing aof file by dump file
    fs::rename(&db_dump_aof, &aof_file).await?;

    // Deleting dump
    if db_dump_aof.exists() {
        let _ = fs::remove_file(&db_dump_aof).await;
    }

    Ok(())
}

pub async fn clean_up_db(db: Db) {
    let mut interval = time::interval(Duration::from_secs(60));

    loop {
        interval.tick().await;
        info!("Cleaning up Database");
        let _ = dump_db_to_aof(&db).await;
    }
}
