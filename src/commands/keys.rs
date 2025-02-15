use crate::commands::utils::{format_int, format_list_response, format_single_response};
use crate::errors::SiderError;
use crate::types::{Command, CommandArgs, Db, DbValue, Key};
use regex::Regex;

pub async fn get_key(db: &Db, command: Command) -> Result<String, SiderError> {
    let key_name = match &command.args {
        CommandArgs::SingleKey(key) => &key.name,
        _ => return Err(SiderError::InvalidCommand),
    };

    let key = {
        let db_read = db.read().await;
        db_read.get(key_name).cloned() // Clone key to release lock
    };

    let key = match key {
        Some(DbValue::StringKey(k)) => k,
        None => return Ok("(nil)\n".to_string()),
        Some(_) => return Err(SiderError::WrongType),
    };

    if let Some(value) = &key.value {
        let deleted = delete_expired_key(db, key.clone()).await; // No read lock at this point

        if !deleted {
            return Ok(format_single_response(value));
        }
    }

    Ok("(nil)\n".to_string())
}

pub async fn set_key(db: &Db, command: Command) -> Result<String, SiderError> {
    let key = match command.args {
        CommandArgs::SingleKey(key) => key,
        _ => return Err(SiderError::InvalidCommand),
    };

    db.write()
        .await
        .insert(key.name.clone(), DbValue::StringKey(key));

    Ok("\"Ok\"\n".to_string())
}

pub async fn delete_key(db: &Db, command: Command) -> Result<String, SiderError> {
    let keys = match &command.args {
        CommandArgs::SingleKey(key) => vec![key.name.clone()],
        CommandArgs::MultipleKeys(keys) => keys.iter().map(|k| k.name.clone()).collect(),
        _ => return Err(SiderError::InvalidCommand),
    };

    let mut db_write = db.write().await;
    let mut deleted_count = 0;

    for key in keys {
        if db_write.swap_remove(&key).is_some() {
            deleted_count += 1;
        }
    }

    Ok(format_int(deleted_count))
}

// Increases the numeric value stored at the key by one.
// If the key does not exist, it is initialized to 0 before
// applying the operation. Returns an error if the key holds
// a non-numeric value or a string that cannot be interpreted
// as an integer.
pub async fn incr(db: &Db, command: Command) -> Result<String, SiderError> {
    incr_decr(db, command, true).await
}

// Decrements the number stored at key by one.
// If the key does not exist, it is set to 0 before performing
// the operation. An error is returned if the key contains a value
// of the wrong type or contains a string that can not be
// represented as integer.
// This operation is limited to 64 bit signed integers.
pub async fn decr(db: &Db, command: Command) -> Result<String, SiderError> {
    incr_decr(db, command, false).await
}

// Increases the number stored at the given key by the specified
// increment. If the key does not exist, it is initialized
// to 0 before applying the operation. Returns an error
// if the key holds a non-numeric value or a string that cannot
// be parsed as a 64-bit signed integer.
pub async fn incrby(db: &Db, command: Command) -> Result<String, SiderError> {
    let key_name = match &command.args {
        CommandArgs::SingleKey(key) => key.name.clone(),
        _ => return Err(SiderError::InvalidCommand),
    };

    let by_str = match &command.args {
        CommandArgs::SingleKey(key) => match key.value.as_deref() {
            Some(by) => by,
            None => return Err(SiderError::NotInt),
        },
        _ => return Err(SiderError::InvalidCommand),
    };

    let by = match by_str.parse::<i64>() {
        Ok(num) => num,
        Err(_) => return Err(SiderError::NotInt),
    };

    let mut db_write = db.write().await;

    let key = match db_write.get_mut(&key_name) {
        Some(DbValue::StringKey(existing_key)) => existing_key,
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
                _ => return Err(SiderError::DatabaseError),
            }
        }
        Some(_) => return Err(SiderError::WrongType),
    };

    let num_str = key.value.as_deref().unwrap_or("0");

    let num = match num_str.parse::<i64>() {
        Ok(n) => n,
        Err(_) => return Err(SiderError::NotInt),
    };

    let new_value = num + by;
    key.value = Some(new_value.to_string());

    Ok(format_int(new_value))
}

async fn incr_decr(db: &Db, command: Command, inc: bool) -> Result<String, SiderError> {
    let key = match command.args {
        CommandArgs::SingleKey(key) => key,
        _ => return Err(SiderError::InvalidCommand),
    };

    let key_name = key.name.clone();

    let mut db_write = db.write().await;

    let key = match db_write.get_mut(&key_name) {
        Some(DbValue::StringKey(key)) => key,
        None => {
            let key = Key::new(key_name.clone(), "0".to_string(), None);
            db_write.insert(key_name.clone(), DbValue::StringKey(key));
            match db_write.get_mut(&key_name) {
                Some(DbValue::StringKey(key)) => key,
                Some(DbValue::ListKey(_)) => return Err(SiderError::WrongType),
                _ => return Err(SiderError::DatabaseError),
            }
        }
        Some(_) => return Err(SiderError::WrongType),
    };

    let Ok(num) = key.value.as_deref().unwrap_or("0").parse::<i64>() else {
        return Err(SiderError::NotInt);
    };

    let new_value = if inc { num + 1 } else { num - 1 };

    key.value = Some(new_value.to_string());

    Ok(format_int(new_value))
}

/// Returns keys matching the Redis-style pattern
pub async fn get_keys(db: &Db, command: Command) -> Result<String, SiderError> {
    let pattern = match &command.args {
        CommandArgs::SingleKey(key) => &key.name,
        _ => return Err(SiderError::InvalidCommand),
    };

    // Convert Redis glob pattern to regex
    let regex_pattern = convert_redis_pattern_to_regex(pattern);
    let re = match Regex::new(&regex_pattern) {
        Ok(re) => re,
        Err(_) => return Err(SiderError::RegexError),
    };

    let mut results = vec![];

    let db_read = db.read().await;

    for key in db_read.keys() {
        if re.is_match(key) {
            results.push(key.clone());
        }
    }

    if results.is_empty() {
        return Ok("(empty array)\n".to_string());
    }

    Ok(format_list_response(results))
}

pub async fn exists(db: &Db, command: Command) -> Result<String, SiderError> {
    let keys = match &command.args {
        CommandArgs::SingleKey(key) => vec![key.name.clone()],
        CommandArgs::MultipleKeys(keys) => keys.iter().map(|k| k.name.clone()).collect(),
        _ => return Err(SiderError::InvalidCommand),
    };

    let db_read = db.read().await;
    let nb_keys = keys.iter().filter(|key| db_read.contains_key(*key)).count() as i64;

    Ok(format_int(nb_keys))
}

pub async fn expire(db: &Db, command: Command) -> Result<String, SiderError> {
    let key = match command.args {
        CommandArgs::SingleKey(key) => key,
        _ => return Err(SiderError::InvalidCommand),
    };

    let ttl = match key.expires_at {
        Some(ttl) => ttl,
        None => return Err(SiderError::TTL),
    };

    let mut db_write = db.write().await;

    match db_write.get_mut(&key.name) {
        Some(DbValue::StringKey(key)) => {
            key.set_ttl(ttl);
            Ok(format_int(1))
        }
        None => Ok(format_int(0)),
        Some(_) => Err(SiderError::WrongType),
    }
}

pub async fn ttl(db: &Db, command: Command) -> Result<String, SiderError> {
    let key = match command.args {
        CommandArgs::SingleKey(key) => key,
        _ => return Err(SiderError::InvalidCommand),
    };

    let db_read = db.read().await;

    let key = match db_read.get(&key.name) {
        Some(DbValue::StringKey(key)) => key,
        None => return Ok(format_int(-2)),
        Some(_) => return Err(SiderError::WrongType),
    };

    Ok(format_int(key.get_ttl()))
}

/// Converts Redis-style glob pattern into a valid regex pattern
// '*' becomes '.*'
// '?' becomes '.'
// '[' stays '[' (range starts)
// ']' stays ']' (range ends)
pub fn convert_redis_pattern_to_regex(pattern: &str) -> String {
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

pub async fn delete_expired_key(db: &Db, key: Key) -> bool {
    let mut db_write = db.write().await;

    if key.is_expired() {
        db_write.swap_remove(&key.name);
        return true;
    }

    false
}
