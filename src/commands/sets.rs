use crate::errors::SiderError;
use crate::response::SiderResponse;
use crate::types::{Command, CommandArgs, Db, DbValue, KeySet};
use std::collections::HashSet;

pub async fn sadd(db: &Db, command: Command) -> Result<SiderResponse, SiderError> {
    let (set_name, values) = match command.args {
        CommandArgs::KeyWithValues { key, values } => (key, values),
        _ => return Err(SiderError::InvalidCommand),
    };

    let mut db_write = db.write().await;

    match db_write.get_mut(&set_name) {
        Some(DbValue::SetKey(db_set)) => {
            db_set.data.extend(values);
            let nb = db_set.data.len() as i64;
            Ok(SiderResponse::Int(nb))
        }
        None => {
            let new_set = DbValue::SetKey(KeySet {
                name: set_name.clone(),
                data: HashSet::from_iter(values.clone()),
                ..Default::default()
            });
            db_write.insert(set_name, new_set);
            let nb = values.len() as i64;
            Ok(SiderResponse::Int(nb))
        }
        Some(_) => Err(SiderError::WrongType),
    }
}

pub async fn smembers(db: &Db, command: Command) -> Result<SiderResponse, SiderError> {
    let key_name = match &command.args {
        CommandArgs::SingleKey(key_name) => key_name,
        _ => return Err(SiderError::InvalidCommand),
    };

    let db_read = db.read().await;

    let results = match db_read.get(key_name) {
        Some(DbValue::SetKey(key)) => key.data.iter().cloned().collect::<Vec<String>>(),
        None => return Ok(SiderResponse::EmptyArray),
        Some(_) => return Err(SiderError::WrongType),
    };

    Ok(SiderResponse::List(results))
}
