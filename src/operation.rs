use crate::aof::write_aof;
use crate::command::Command;
use crate::database::Db;

pub async fn pong() -> String {
    "PONG\n".to_string()
}

pub async fn get_key(db: &Db, command: Command) -> String {
    let db_read = db.read().await;
    match db_read.get(&command.keys[0]) {
        Some(value) => format!("{}\n", value),
        _ => "nil\n".to_string(),
    }
}

pub async fn set_key(db: &Db, command: Command) -> String {
    let key = command.keys[0].clone();
    let value = command.value.clone().unwrap();
    db.write().await.insert(key.clone(), value.clone());

    write_aof(command)
        .await
        .expect("Error writing to AOF file!");

    "OK\n".to_string()
}

pub async fn delete_key(db: &Db, command: Command) -> String {
    let key = command.keys[0].clone();
    let mut db_write = db.write().await;
    match db_write.remove(&key) {
        Some(_) => format!("OK\n"),
        _ => "nil\n".to_string(),
    }
}

pub async fn flush_db(db: &Db) -> String {
    db.write().await.clear();
    "OK\n".to_string()
}
