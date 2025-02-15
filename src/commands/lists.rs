use crate::commands::utils::{format_int, format_list_response};
use crate::errors::{SiderError, SiderErrorKind};
use crate::types::{Command, CommandArgs, Db, DbValue, KeyList, ListPushType, PopType};

pub async fn push_to_list(
    db: &Db,
    command: Command,
    push_type: ListPushType,
) -> Result<String, SiderError> {
    let key_list = match command.args {
        CommandArgs::KeyWithValues(key) => key,
        _ => return Err(SiderError::new(SiderErrorKind::InvalidCommand)),
    };

    let key_name = key_list.name.clone();
    let mut new_values = key_list.values.clone();

    let mut db_write = db.write().await;

    match db_write.get_mut(&key_name) {
        Some(DbValue::ListKey(existing_list)) => {
            match push_type {
                ListPushType::LPUSH => {
                    new_values.reverse();
                    existing_list.values.splice(0..0, new_values);
                }
                ListPushType::RPUSH => {
                    existing_list.values.extend(new_values);
                }
            }
            let nb = existing_list.values.len() as i64;
            Ok(format_int(nb))
        }
        None => {
            if let ListPushType::LPUSH = push_type {
                new_values.reverse();
            }
            db_write.insert(
                key_name.clone(),
                DbValue::ListKey(KeyList {
                    name: key_name,
                    values: new_values.clone(),
                }),
            );
            let nb = new_values.len() as i64;
            Ok(format_int(nb))
        }
        Some(_) => Err(SiderError::new(SiderErrorKind::WrongType)),
    }
}

pub async fn lpush(db: &Db, command: Command) -> Result<String, SiderError> {
    push_to_list(db, command, ListPushType::LPUSH).await
}

pub async fn rpush(db: &Db, command: Command) -> Result<String, SiderError> {
    push_to_list(db, command, ListPushType::RPUSH).await
}

pub async fn lrange(db: &Db, command: Command) -> Result<String, SiderError> {
    let key_list = match command.args {
        CommandArgs::KeyWithValues(key) => key,
        _ => return Err(SiderError::new(SiderErrorKind::InvalidCommand)),
    };

    let key_name = key_list.name.clone();

    let min: isize = match key_list.values[0].parse::<isize>() {
        Ok(val) => val,
        Err(_) => {
            return Err(SiderError::with_message(
                SiderErrorKind::Custom,
                "(error) value is not an integer or out of range",
            ))
        }
    };

    let max: isize = match key_list.values[1].parse::<isize>() {
        Ok(val) => val,
        Err(_) => {
            return Err(SiderError::with_message(
                SiderErrorKind::Custom,
                "(error) value is not an integer or out of range",
            ))
        }
    };

    let db_read = db.read().await;

    let key = match db_read.get(&key_name) {
        Some(DbValue::ListKey(key)) => key,
        Some(DbValue::StringKey(_)) => return Err(SiderError::new(SiderErrorKind::WrongType)),
        _ => return Err(SiderError::new(SiderErrorKind::EmptyArray)),
    };

    let len = key.values.len();

    let min = if min >= 0 {
        min as usize
    } else {
        (len as isize + min).max(0) as usize
    };

    let max = if max >= 0 {
        (max + 1).min(len as isize) as usize
    } else {
        (len as isize + max + 1).max(0).min(len as isize) as usize
    };

    if min >= max || min >= len {
        return Err(SiderError::new(SiderErrorKind::EmptyArray));
    }

    let results: &[String] = &key.values[min..max];

    if results.is_empty() {
        return Err(SiderError::new(SiderErrorKind::EmptyArray));
    }

    Ok(format_list_response(results.to_vec()))
}

pub async fn lpop(db: &Db, command: Command) -> Result<String, SiderError> {
    pop_list(db, command, PopType::LPOP).await
}

pub async fn rpop(db: &Db, command: Command) -> Result<String, SiderError> {
    pop_list(db, command, PopType::RPOP).await
}

async fn pop_list(db: &Db, command: Command, pop_type: PopType) -> Result<String, SiderError> {
    let key = match &command.args {
        CommandArgs::SingleKey(key) => key,
        _ => return Err(SiderError::new(SiderErrorKind::InvalidCommand)),
    };

    let key_name = key.name.clone();

    let mut db_write = db.write().await;

    let key_db = match db_write.get_mut(&key_name) {
        Some(DbValue::ListKey(key)) => key,
        Some(DbValue::StringKey(_)) => return Err(SiderError::new(SiderErrorKind::WrongType)),
        _ => return Err(SiderError::new(SiderErrorKind::EmptyArray)),
    };

    let nb = key
        .value
        .as_deref()
        .unwrap_or("1")
        .parse::<usize>()
        .unwrap_or(1);

    let len = key_db.values.len();

    let (start, end) = match pop_type {
        PopType::LPOP => (0, nb),
        PopType::RPOP => (len.saturating_sub(nb), len),
    };

    let mut removed: Vec<String> = key_db
        .values
        .drain(start..end.min(key_db.values.len()))
        .collect();

    if removed.is_empty() {
        return Err(SiderError::new(SiderErrorKind::Nil));
    }

    if let PopType::RPOP = pop_type {
        removed.reverse();
    }

    Ok(format_list_response(removed))
}
