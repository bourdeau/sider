use crate::aof::delete_aof_file;
use crate::database::Db;
use crate::types::Command;
use crate::types::CommandArgs;
use crate::types::DbValue;
use crate::types::Key;
use crate::types::KeyList;
use regex::Regex;

const ERROR_LIST_KEY: &str =
    "(error) WRONGTYPE Operation against a key holding the wrong kind of value\n";

async fn delete_expired_key(db: &Db, key: Key) -> bool {
    let mut db_write = db.write().await;

    if key.is_expired() {
        db_write.remove(&key.name);
        return true;
    }

    false
}

pub async fn pong() -> String {
    "PONG\n".to_string()
}

pub async fn get_key(db: &Db, command: Command) -> String {
    let key_name = match &command.args {
        CommandArgs::SingleKey(key) => &key.name,
        _ => return "ERR invalid command\n".to_string(),
    };

    let key = {
        let db_read = db.read().await;
        db_read.get(key_name).cloned() // Clone key to release lock
    };

    let nil = "(nil)\n".to_string();

    let key = match key {
        Some(DbValue::StringKey(k)) => k,
        Some(DbValue::ListKey(_)) => return ERROR_LIST_KEY.to_string(),
        None => return nil,
    };

    if let Some(value) = &key.value {
        let deleted = delete_expired_key(db, key.clone()).await; // No read lock at this point

        if !deleted {
            return format!("{}\n", value);
        }
    }

    nil
}

pub async fn set_key(db: &Db, command: Command) -> String {
    let key = match command.args {
        CommandArgs::SingleKey(key) => key,
        _ => return "ERR invalid command\n".to_string(),
    };

    db.write()
        .await
        .insert(key.name.clone(), DbValue::StringKey(key));

    "OK\n".to_string()
}

pub async fn delete_key(db: &Db, command: Command) -> String {
    let keys = match &command.args {
        CommandArgs::SingleKey(key) => vec![key.name.clone()],
        CommandArgs::MultipleKeys(keys) => keys.iter().map(|k| k.name.clone()).collect(),
        _ => return "ERR invalid command\n".to_string(),
    };

    let mut db_write = db.write().await;
    let mut deleted_count = 0;

    for key in keys {
        if db_write.remove(&key).is_some() {
            deleted_count += 1;
        }
    }

    format!("(integer) {}\n", deleted_count)
}

pub async fn flush_db(db: &Db) -> String {
    db.write().await.clear();
    // delete aof file
    delete_aof_file().await;
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
    let pattern = match &command.args {
        CommandArgs::SingleKey(key) => &key.name,
        _ => return "ERR invalid command\n".to_string(),
    };

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

    if results.is_empty() {
        return "(empty array)\n".to_string();
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
    let keys = match &command.args {
        CommandArgs::SingleKey(key) => vec![key.name.clone()],
        CommandArgs::MultipleKeys(keys) => keys.iter().map(|k| k.name.clone()).collect(),
        _ => return "ERR invalid command\n".to_string(),
    };

    let db_read = db.read().await;
    let nb_keys = keys.iter().filter(|key| db_read.contains_key(*key)).count();

    format!("{}\n", nb_keys)
}

pub async fn expire(db: &Db, command: Command) -> String {
    let key = match command.args {
        CommandArgs::SingleKey(key) => key,
        _ => return "ERR invalid command\n".to_string(),
    };

    let ttl = match key.expires_at {
        Some(ttl) => ttl,
        None => return "Error: TTL is required\n".to_string(),
    };

    let mut db_write = db.write().await;

    match db_write.get_mut(&key.name) {
        Some(DbValue::ListKey(_)) => "ERR key is a list, not a string\n".to_string(),
        Some(DbValue::StringKey(key)) => {
            key.set_ttl(ttl);
            "(integer) 1\n".to_string()
        }
        None => "(integer) 0\n".to_string(),
    }
}

pub async fn ttl(db: &Db, command: Command) -> String {
    let key = match command.args {
        CommandArgs::SingleKey(key) => key,
        _ => return "ERR invalid command\n".to_string(),
    };

    let db_read = db.read().await;

    let key = match db_read.get(&key.name) {
        Some(DbValue::StringKey(key)) => key,
        Some(DbValue::ListKey(_)) => return "ERR key is a list, not a string\n".to_string(),
        None => return "(integer) -2\n".to_string(),
    };

    format!("(integer) {}\n", key.get_ttl())
}

pub async fn incr_decr(db: &Db, command: Command, inc: bool) -> String {
    let key = match command.args {
        CommandArgs::SingleKey(key) => key,
        _ => return "ERR invalid command\n".to_string(),
    };

    let key_name = key.name.clone();

    let mut db_write = db.write().await;

    let key = match db_write.get_mut(&key_name) {
        Some(DbValue::StringKey(key)) => key,
        Some(DbValue::ListKey(_)) => return "ERR key is a list, not a string\n".to_string(),
        None => {
            let key = Key::new(key_name.clone(), "0".to_string(), None);
            db_write.insert(key_name.clone(), DbValue::StringKey(key));
            match db_write.get_mut(&key_name) {
                Some(DbValue::StringKey(key)) => key,
                Some(DbValue::ListKey(_)) => {
                    return "ERR key is a list, not a string\n".to_string()
                }
                _ => return "ERR unexpected database error\n".to_string(),
            }
        }
    };

    let Ok(num) = key.value.as_deref().unwrap_or("0").parse::<i64>() else {
        return "ERR value is not an integer\n".to_string();
    };

    let new_value = if inc { num + 1 } else { num - 1 };

    key.value = Some(new_value.to_string());

    format!("(integer) {}\n", new_value)
}

// Increases the numeric value stored at the key by one.
// If the key does not exist, it is initialized to 0 before
// applying the operation. Returns an error if the key holds
// a non-numeric value or a string that cannot be interpreted
// as an integer.
pub async fn incr(db: &Db, command: Command) -> String {
    incr_decr(db, command, true).await
}

// Decrements the number stored at key by one.
// If the key does not exist, it is set to 0 before performing
// the operation. An error is returned if the key contains a value
// of the wrong type or contains a string that can not be
// represented as integer.
// This operation is limited to 64 bit signed integers.
pub async fn decr(db: &Db, command: Command) -> String {
    incr_decr(db, command, false).await
}

// Increases the number stored at the given key by the specified
// increment. If the key does not exist, it is initialized
// to 0 before applying the operation. Returns an error
// if the key holds a non-numeric value or a string that cannot
// be parsed as a 64-bit signed integer.
pub async fn incrby(db: &Db, command: Command) -> String {
    let key_name = match &command.args {
        CommandArgs::SingleKey(key) => key.name.clone(),
        _ => return "ERR invalid command\n".to_string(),
    };

    let by_str = match &command.args {
        CommandArgs::SingleKey(key) => match key.value.as_deref() {
            Some(by) => by,
            None => return "ERR value is not an integer\n".to_string(),
        },
        _ => return "ERR invalid command\n".to_string(),
    };

    let by = match by_str.parse::<i64>() {
        Ok(num) => num,
        Err(_) => return "ERR value is not an integer\n".to_string(),
    };

    let mut db_write = db.write().await;

    let key = match db_write.get_mut(&key_name) {
        Some(DbValue::StringKey(existing_key)) => existing_key,
        Some(_) => return "ERR key is not a string\n".to_string(),
        None => {
            db_write.insert(
                key_name.clone(),
                DbValue::StringKey(Key {
                    name: key_name.clone(),
                    value: Some("0".to_string()),
                    ..Default::default()
                }),
            );
            match db_write.get_mut(&key_name) {
                Some(DbValue::StringKey(new_key)) => new_key,
                _ => return "ERR unexpected database error\n".to_string(),
            }
        }
    };

    let num_str = key.value.as_deref().unwrap_or("0");

    let num = match num_str.parse::<i64>() {
        Ok(n) => n,
        Err(_) => return "ERR value is not an integer\n".to_string(),
    };

    let new_value = num + by;
    key.value = Some(new_value.to_string());

    format!("(integer) {}\n", new_value)
}

pub async fn lpush(db: &Db, command: Command) -> String {
    let key_list = match command.args {
        CommandArgs::KeyWithValues(key) => key,
        _ => return "ERR invalid command\n".to_string(),
    };

    let key_name = key_list.name.clone();
    let new_values = key_list.values.clone();

    let mut db_write = db.write().await;

    match db_write.get_mut(&key_name) {
        Some(DbValue::ListKey(existing_list)) => {
            existing_list.values.splice(0..0, new_values);
            format!("(integer) {}\n", existing_list.values.len())
        }
        Some(_) => "ERR key exists as non-list type\n".to_string(),
        None => {
            db_write.insert(
                key_name.clone(),
                DbValue::ListKey(KeyList {
                    name: key_name,
                    values: new_values.clone(),
                }),
            );
            format!("(integer) {}\n", new_values.len())
        }
    }
}
