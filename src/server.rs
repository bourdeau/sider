use crate::database::Db;
use crate::process::process_command;

use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::info;

pub async fn handle_client(mut socket: TcpStream, db: Db) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = socket.read(&mut buffer).await?;

        if bytes_read == 0 {
            info!("Client disconnected");
            return Ok(());
        }

        let command = String::from_utf8_lossy(&buffer[..bytes_read])
            .trim()
            .to_string();

        info!("{}", command);

        let response = process_command(command, &db, false).await;

        socket.write_all(response.as_bytes()).await?;
    }
}
