use crate::errors::SiderError;
use crate::response::SiderResponse;
use crate::types::Command;
use crate::types::CommandArgs;
use crate::types::Db;
use crate::types::DbValue;
use crate::types::KeyHash;

pub async fn hset(db: &Db, command: Command) -> Result<SiderResponse, SiderError> {
    let key = match &command.args {
        CommandArgs::HashKey(key) => key,
        _ => return Err(SiderError::InvalidCommand),
    };

    let key_name = key.name.clone();
    let key_values = key.fields.clone();

    let key = {
        let db_read = db.read().await;
        db_read.get(&key_name).cloned()
    };

    let mut db_write = db.write().await;

    let nb = match key {
        Some(DbValue::HashKey(mut k)) => {
            let before_len = k.fields.len();
            k.fields.extend(key_values.clone());
            let after_len = k.fields.len();
            db_write.insert(key_name, DbValue::HashKey(k));
            after_len - before_len
        }
        None => {
            db_write.insert(
                key_name.clone(),
                DbValue::HashKey(KeyHash {
                    name: key_name,
                    fields: key_values.clone(),
                }),
            );
            key_values.len()
        }
        Some(_) => return Err(SiderError::WrongType),
    };

    Ok(SiderResponse::Int(nb as i64))
}

pub async fn hget(db: &Db, command: Command) -> Result<SiderResponse, SiderError> {
    let (hash_name, field_name) = match &command.args {
        CommandArgs::HashField(hash) => (&hash.key, &hash.field),
        _ => return Err(SiderError::InvalidCommand),
    };

    let db_read = db.read().await;

    match db_read.get(hash_name) {
        Some(DbValue::HashKey(hash)) => match hash.fields.get(field_name) {
            Some(value) => Ok(SiderResponse::SimpleString(value.to_string())),
            None => Ok(SiderResponse::Nil),
        },
        None => Ok(SiderResponse::Nil),
        Some(_) => Err(SiderError::WrongType),
    }
}

pub async fn hgetall(db: &Db, command: Command) -> Result<SiderResponse, SiderError> {
    let key_name = match &command.args {
        CommandArgs::KeyName(name) => name,
        _ => return Err(SiderError::InvalidCommand),
    };

    let db_read = db.read().await;

    let results = match db_read.get(key_name) {
        Some(DbValue::HashKey(hash)) => hash
            .fields
            .iter()
            .flat_map(|(k, v)| vec![k.clone(), v.clone()])
            .collect::<Vec<String>>(),
        Some(_) => return Err(SiderError::WrongType),
        None => return Ok(SiderResponse::EmptyArray),
    };

    Ok(SiderResponse::List(results))
}

pub async fn hdel(db: &Db, command: Command) -> Result<SiderResponse, SiderError> {
    let key = match &command.args {
        CommandArgs::KeyWithValues(key) => key,
        _ => return Err(SiderError::InvalidCommand),
    };

    let key_name = key.name.clone();
    let fields = key.values.clone();

    let mut db_write = db.write().await;

    match db_write.get_mut(&key_name) {
        Some(DbValue::HashKey(hash)) => {
            let mut deleted_count = 0;
            for field in fields {
                if hash.fields.swap_remove(&field).is_some() {
                    deleted_count += 1;
                }
            }

            if hash.fields.is_empty() {
                db_write.swap_remove(&key_name);
            }
            Ok(SiderResponse::Int(deleted_count))
        }
        Some(_) => Err(SiderError::WrongType),
        None => Ok(SiderResponse::Int(0)),
    }
}
