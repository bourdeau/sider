use dirs::home_dir;
use std::fs;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::command::Command;

fn get_aof_log_dir() -> PathBuf {
    let home = home_dir().expect("Failed to get home directory");
    home.join(".local/share/sider")
}

pub async fn write_aof(command: Command) -> std::io::Result<()> {
    let log_path = get_aof_log_dir();

    if !log_path.exists() {
        fs::create_dir_all(&log_path)?;
    }

    // todo: handle command value error, should't crash the program
    let command_value = match command.value {
        Some(value) => value,
        None => panic!("Command value is required"),
    };

    let formatted = format!(
        "{:?} {:?} {} \n",
        command.command_type, command.keys, command_value
    );

    let file_path = log_path.join("appendonly.aof");

    let mut file = File::options()
        .append(true)
        .create(true)
        .open(&file_path)
        .await?;
    file.write_all(formatted.as_bytes()).await?;

    Ok(())
}
