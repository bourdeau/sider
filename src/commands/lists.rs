use crate::errors::SiderError;
use crate::response::SiderResponse;
use crate::types::{Command, CommandArgs, Db, DbValue, KeyList, ListPushType, PopType};

pub async fn push_to_list(
    db: &Db,
    command: Command,
    push_type: ListPushType,
) -> Result<SiderResponse, SiderError> {
    let (key_name, values) = match command.args {
        CommandArgs::KeyWithValues { key, values } => (key, values),
        _ => return Err(SiderError::InvalidCommand),
    };

    let mut new_values = values.clone();

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
            Ok(SiderResponse::Int(nb))
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
            Ok(SiderResponse::Int(nb))
        }
        Some(_) => Err(SiderError::WrongType),
    }
}

pub async fn lpush(db: &Db, command: Command) -> Result<SiderResponse, SiderError> {
    push_to_list(db, command, ListPushType::LPUSH).await
}

pub async fn rpush(db: &Db, command: Command) -> Result<SiderResponse, SiderError> {
    push_to_list(db, command, ListPushType::RPUSH).await
}

pub async fn lrange(db: &Db, command: Command) -> Result<SiderResponse, SiderError> {
    let (key_name, values) = match command.args {
        CommandArgs::KeyWithValues { key, values } => (key, values),
        _ => return Err(SiderError::InvalidCommand),
    };

    let min: isize = match values[0].parse::<isize>() {
        Ok(val) => val,
        Err(_) => return Err(SiderError::NotIntOrOutOfRange),
    };

    let max: isize = match values[1].parse::<isize>() {
        Ok(val) => val,
        Err(_) => return Err(SiderError::NotIntOrOutOfRange),
    };

    let db_read = db.read().await;

    let key = match db_read.get(&key_name) {
        Some(DbValue::ListKey(key)) => key,
        Some(DbValue::StringKey(_)) => return Err(SiderError::WrongType),
        _ => return Ok(SiderResponse::EmptyArray),
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
        return Ok(SiderResponse::EmptyArray);
    }

    let results: &[String] = &key.values[min..max];

    if results.is_empty() {
        return Ok(SiderResponse::EmptyArray);
    }

    Ok(SiderResponse::List(results.to_vec()))
}

pub async fn lpop(db: &Db, command: Command) -> Result<SiderResponse, SiderError> {
    pop_list(db, command, PopType::LPOP).await
}

pub async fn rpop(db: &Db, command: Command) -> Result<SiderResponse, SiderError> {
    pop_list(db, command, PopType::RPOP).await
}

async fn pop_list(
    db: &Db,
    command: Command,
    pop_type: PopType,
) -> Result<SiderResponse, SiderError> {
    let (key_name, value) = match &command.args {
        CommandArgs::SingleKey(key) => (key.clone(), None),
        CommandArgs::KeyWithValue { key, value } => (key.clone(), Some(value.clone())),
        _ => return Err(SiderError::InvalidCommand),
    };

    let mut db_write = db.write().await;

    let key_db = match db_write.get_mut(&key_name) {
        Some(DbValue::ListKey(key)) => key,
        Some(DbValue::StringKey(_)) => return Err(SiderError::WrongType),
        _ => return Ok(SiderResponse::EmptyArray),
    };

    let nb = value
        .as_deref()
        .map_or(1, |v| v.parse::<usize>().unwrap_or(1));

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
        return Ok(SiderResponse::Nil);
    }

    if let PopType::RPOP = pop_type {
        removed.reverse();
    }

    // If LPOP is passed without arguments, return the first element
    if nb == 1 {
        return Ok(SiderResponse::SimpleString(removed[0].clone()));
    }

    Ok(SiderResponse::List(removed))
}
