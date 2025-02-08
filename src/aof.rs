use dirs::home_dir;
use std::fs;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::types::{Command, CommandArgs};

pub fn get_aof_log_dir() -> PathBuf {
    let home = home_dir().expect("Failed to get home directory");
    home.join(".local/share/sider")
}

pub async fn write_aof(command: &Command) -> std::io::Result<()> {
    let log_path = get_aof_log_dir();

    if !log_path.exists() {
        fs::create_dir_all(&log_path)?;
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
