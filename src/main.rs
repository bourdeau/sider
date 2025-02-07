use sider::database::Db;
use sider::server::handle_client;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

use sider::task::delete_expired_keys;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db: Db = Arc::new(RwLock::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Listening on 127.0.0.1:6379...");

    // Background task to delete expired keys
    // cloning Arc is cheap apparently
    tokio::spawn(delete_expired_keys(db.clone()));

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New client connected: {}", addr);

        // Clone the Arc for the new task
        let db = Arc::clone(&db);

        // Spawn a task to handle each client
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, db).await {
                eprintln!("Error handling client {}: {:?}", addr, e);
            }
        });
    }
}
