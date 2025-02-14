use crate::commands::utils::format_list_response;
use crate::commands::utils::ERROR_KEY_TYPE;
use crate::types::Command;
use crate::types::CommandArgs;
use crate::types::Db;
use crate::types::DbValue;
use crate::types::KeyHash;

pub async fn hset(db: &Db, command: Command) -> String {
    let key = match &command.args {
        CommandArgs::HashKey(key) => key,
        _ => return "ERR invalid command\n".to_string(),
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
        Some(_) => return ERROR_KEY_TYPE.to_string(),
    };

    format!("(integer) {}\n", nb)
}

pub async fn hget(db: &Db, command: Command) -> String {
    let (hash_name, field_name) = match &command.args {
        CommandArgs::HashField(hash) => (&hash.key, &hash.field),
        _ => return "ERR invalid command\n".to_string(),
    };

    let db_read = db.read().await;

    match db_read.get(hash_name) {
        Some(DbValue::HashKey(hash)) => match hash.fields.get(field_name) {
            Some(value) => format!("{}\n", value),
            None => "(nil)\n".to_string(),
        },
        None => "(nil)\n".to_string(),
        Some(_) => "ERR wrong type\n".to_string(),
    }
}

pub async fn hgetall(db: &Db, command: Command) -> String {
    let key_name = match &command.args {
        CommandArgs::KeyName(name) => name,
        _ => return "ERR invalid command\n".to_string(),
    };

    let db_read = db.read().await;

    let results = match db_read.get(key_name) {
        Some(DbValue::HashKey(hash)) => hash
            .fields
            .iter()
            .map(|(k, v)| format!("{} {}", k, v))
            .collect::<Vec<String>>(),
        Some(_) => return "ERR wrong type\n".to_string(),
        None => return "(empty array)\n".to_string(),
    };

    format_list_response(results)
}
