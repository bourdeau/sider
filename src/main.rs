use indexmap::IndexMap;
use sider::aof::clean_up_db;
use sider::config::get_config;
use sider::database::delete_expired_keys;
use sider::database::restore_from_aof;
use sider::server::handle_client;
use sider::types::Db;
use std::error::Error;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let db: Db = Arc::new(RwLock::new(IndexMap::new()));

    // Config
    let config = get_config();
    let port: u16 = config.get("port").expect("Port is missing");
    let bind: Ipv4Addr = config.get("bind").expect("Bind is missing");
    let full_address = format!("{}:{}", bind, port);

    let listener = TcpListener::bind(full_address.to_string()).await?;
    let message = format!("Listening {}...", full_address);

    info!(message);

    // Restoring DB from AOF file at start up
    tokio::spawn(restore_from_aof(db.clone()));

    // Delete expired keys every 60 seconds
    tokio::spawn(delete_expired_keys(db.clone()));

    // Clean database every 60 seconds
    tokio::spawn(clean_up_db(db.clone()));

    loop {
        let (socket, addr) = listener.accept().await?;
        info!("New client connected: {}", addr);

        // Clone the Arc for the new task
        let db = Arc::clone(&db);

        // Spawn a task to handle each client
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, db).await {
                error!("Error handling client {}: {:?}", addr, e);
            }
        });
    }
}
