use regex::Regex;
use crate::aof::write_aof;
use crate::command::Command;
use crate::database::Db;
use crate::command::Key;

pub async fn pong() -> String {
    "PONG\n".to_string()
}

pub async fn get_key(db: &Db, command: Command) -> String {
    let db_read = db.read().await;
    match db_read.get(&command.keys[0].name) {
        Some(key) => match &key.value {
            Some(val) => format!("{}\n", val),
            None => "nil\n".to_string(),
        },
        None => "nil\n".to_string(),
    }
}

pub async fn set_key(db: &Db, command: Command) -> String {
    let key: Key = command.keys[0].clone();

    db.write().await.insert(key.name.clone(), key);

    write_aof(command)
        .await
        .expect("Error writing to AOF file!");

    "OK\n".to_string()
}

pub async fn delete_key(db: &Db, command: Command) -> String {
    let key: Key = command.keys[0].clone();
    let mut db_write = db.write().await;
    match db_write.remove(&key.name) {
        Some(_) => "OK\n".to_string(),
        _ => "nil\n".to_string(),
    }
}

pub async fn flush_db(db: &Db) -> String {
    db.write().await.clear();
    "OK\n".to_string()
}

/// Converts Redis-style glob pattern into a valid regex pattern
// '*' becomes '.*'
// '?' becomes '.'
// '[' stays '[' (range starts)
// ']' stays ']' (range ends)
fn convert_redis_pattern_to_regex(pattern: &str) -> String {
    let mut regex_pattern = String::from("^");

    for c in pattern.chars() {
        match c {
            '*' => regex_pattern.push_str(".*"),
            '?' => regex_pattern.push('.'),
            '[' => regex_pattern.push('['),
            ']' => regex_pattern.push(']'),
            _ => regex_pattern.push_str(&regex::escape(&c.to_string())), // Escape other chars
        }
    }

    regex_pattern.push('$');
    regex_pattern
}

/// Returns keys matching the Redis-style pattern
pub async fn get_keys(db: &Db, command: Command) -> String {
    let pattern = command.keys[0].name.as_str();

    // Convert Redis glob pattern to regex
    let regex_pattern = convert_redis_pattern_to_regex(pattern);
    let re = match Regex::new(&regex_pattern) {
        Ok(re) => re,
        Err(e) => return format!("Error: {}\n", e),
    };

    let mut results = vec![];

    let db_read = db.read().await;
    for key in db_read.keys() {
        if re.is_match(key) {
            results.push(key.clone());
        }
    }

    results
        .iter()
        .enumerate()
        .map(|(i, key)| format!("{}) \"{}\"", i + 1, key))
        .collect::<Vec<String>>()
        .join("\n")
        + "\n"
}

pub async fn exists(db: &Db, command: Command) -> String {
    let db_read = db.read().await;
    let nb_keys = command
        .keys
        .iter()
        .filter(|key| db_read.contains_key(&key.name))
        .count();
    
    format!("{}\n", nb_keys)
}

pub async fn expire(db: &Db, command: Command) -> String {
    let key = match command.keys.first() {
        Some(key) => key,
        None => return "Error: No key provided\n".to_string(),
    };

    let ttl = match key.expires_at {
        Some(ttl) => ttl,
        None => return "Error: TTL is required\n".to_string(),
    };

    let mut db_write = db.write().await;

    match db_write.get_mut(&key.name) {
        Some(key) => {
            key.expires_at = Some(ttl);
            "(integer) 1\n".to_string()
        }
        None => "(integer) 0\n".to_string(),
    }
}
