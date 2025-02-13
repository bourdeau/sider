use crate::commands::utils::{format_list_response, ERROR_KEY_TYPE};
use crate::types::{Command, CommandArgs, Db, DbValue, KeyList, ListPushType, PopType};

pub async fn push_to_list(db: &Db, command: Command, push_type: ListPushType) -> String {
    let key_list = match command.args {
        CommandArgs::KeyWithValues(key) => key,
        _ => return "ERR invalid command\n".to_string(),
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
            format!("(integer) {}\n", existing_list.values.len())
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
            format!("(integer) {}\n", new_values.len())
        }
        Some(_) => ERROR_KEY_TYPE.to_string(),
    }
}

pub async fn lpush(db: &Db, command: Command) -> String {
    push_to_list(db, command, ListPushType::LPUSH).await
}

pub async fn rpush(db: &Db, command: Command) -> String {
    push_to_list(db, command, ListPushType::RPUSH).await
}

pub async fn lrange(db: &Db, command: Command) -> String {
    let key_list = match command.args {
        CommandArgs::KeyWithValues(key) => key,
        _ => return "ERR invalid command\n".to_string(),
    };

    let key_name = key_list.name.clone();

    let min: isize = match key_list.values[0].parse::<isize>() {
        Ok(val) => val,
        Err(_) => return "(error) value is not an integer or out of range\n".to_string(),
    };

    let max: isize = match key_list.values[1].parse::<isize>() {
        Ok(val) => val,
        Err(_) => return "(error) value is not an integer or out of range\n".to_string(),
    };

    let db_read = db.read().await;

    let key = match db_read.get(&key_name) {
        Some(DbValue::ListKey(key)) => key,
        Some(DbValue::StringKey(_)) => return ERROR_KEY_TYPE.to_string(),
        _ => return "(empty array)\n".to_string(),
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
        return "(empty array)\r\n".to_string();
    }

    let results: &[String] = &key.values[min..max];

    if results.is_empty() {
        return "(empty array)\n".to_string();
    }

    format_list_response(results.to_vec())
}

pub async fn lpop(db: &Db, command: Command) -> String {
    pop_list(db, command, PopType::LPOP).await
}

pub async fn rpop(db: &Db, command: Command) -> String {
    pop_list(db, command, PopType::RPOP).await
}

async fn pop_list(db: &Db, command: Command, pop_type: PopType) -> String {
    let key = match &command.args {
        CommandArgs::SingleKey(key) => key,
        _ => return "ERR invalid command\n".to_string(),
    };

    let key_name = key.name.clone();

    let mut db_write = db.write().await;

    let key_db = match db_write.get_mut(&key_name) {
        Some(DbValue::ListKey(key)) => key,
        Some(DbValue::StringKey(_)) => return ERROR_KEY_TYPE.to_string(),
        _ => return "(empty array)\n".to_string(),
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
        return "(nil)\n".to_string();
    }

    if let PopType::RPOP = pop_type {
        removed.reverse();
    }

    format_list_response(removed)
}
