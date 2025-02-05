use crate::command::{parse_command, CommandType};
use crate::database::Db;
use crate::operation::{delete_key, flush_db, get_key, get_keys, pong, set_key};
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn handle_client(mut socket: TcpStream, db: Db) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = socket.read(&mut buffer).await?;

        if bytes_read == 0 {
            println!("Client disconnected");
            return Ok(());
        }

        let received_data = String::from_utf8_lossy(&buffer[..bytes_read])
            .trim()
            .to_string();

        let response = process(received_data, &db).await;

        socket.write_all(response.as_bytes()).await?;
    }
}

async fn process(command: String, db: &Db) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        return "ERROR: Empty command\n".to_string();
    }

    let command = match parse_command(&command) {
        Ok(cmd) => cmd,
        Err(e) => return format!("ERROR: {}\n", e),
    };

    match command.command_type {
        CommandType::PONG => pong().await,
        CommandType::GET => get_key(db, command).await,
        CommandType::SET => set_key(db, command).await,
        CommandType::DELETE => delete_key(db, command).await,
        CommandType::FLUSHDB => flush_db(db).await,
        CommandType::KEYS => get_keys(db, command).await,
    }
}
