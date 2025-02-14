use indexmap::IndexMap;

use crate::types::{Command, CommandArgs, CommandType, HashField, Key, KeyHash, KeyList};

pub fn build_pong_command() -> Command {
    Command {
        command_type: CommandType::PONG,
        args: CommandArgs::SingleKey(Key::default()),
    }
}

pub fn build_flush_db_command() -> Command {
    Command {
        command_type: CommandType::FLUSHDB,
        args: CommandArgs::SingleKey(Key::default()),
    }
}

pub fn build_get_command(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err("ERR wrong number of arguments".to_string());
    }
    Ok(Command {
        command_type: CommandType::GET,
        args: CommandArgs::SingleKey(Key {
            name: args[0].to_string(),
            ..Default::default()
        }),
    })
}

pub fn build_keys_command(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err("ERR wrong number of arguments".to_string());
    }
    Ok(Command {
        command_type: CommandType::KEYS,
        args: CommandArgs::SingleKey(Key {
            name: args[0].to_string(),
            ..Default::default()
        }),
    })
}

pub fn build_set_command(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Err("ERR wrong number of arguments".to_string());
    }
    Ok(Command {
        command_type: CommandType::SET,
        args: CommandArgs::SingleKey(Key::new(args[0].to_string(), args[1].to_string(), None)),
    })
}

pub fn build_delete_command(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err("ERR wrong number of arguments".to_string());
    }
    Ok(Command {
        command_type: CommandType::DEL,
        args: CommandArgs::MultipleKeys(
            args.iter()
                .map(|key| Key {
                    name: key.to_string(),
                    ..Default::default()
                })
                .collect(),
        ),
    })
}

pub fn build_exists_command(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err("ERR wrong number of arguments".to_string());
    }
    Ok(Command {
        command_type: CommandType::EXISTS,
        args: CommandArgs::MultipleKeys(
            args.iter()
                .map(|key| Key {
                    name: key.to_string(),
                    ..Default::default()
                })
                .collect(),
        ),
    })
}

pub fn build_expire_command(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Err("ERR wrong number of arguments".to_string());
    }
    let ttl = args[1]
        .parse::<i64>()
        .map_err(|_| "ERR value is not an integer".to_string())?;
    Ok(Command {
        command_type: CommandType::EXPIRE,
        args: CommandArgs::SingleKey(Key {
            name: args[0].to_string(),
            expires_at: Some(ttl),
            ..Default::default()
        }),
    })
}

pub fn build_ttl_command(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err("ERR wrong number of arguments".to_string());
    }
    Ok(Command {
        command_type: CommandType::TTL,
        args: CommandArgs::SingleKey(Key {
            name: args[0].to_string(),
            ..Default::default()
        }),
    })
}

pub fn build_incr_command(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err("ERR wrong number of arguments".to_string());
    }
    Ok(Command {
        command_type: CommandType::INCR,
        args: CommandArgs::SingleKey(Key {
            name: args[0].to_string(),
            ..Default::default()
        }),
    })
}

pub fn build_decr_command(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err("ERR wrong number of arguments".to_string());
    }
    Ok(Command {
        command_type: CommandType::DECR,
        args: CommandArgs::SingleKey(Key {
            name: args[0].to_string(),
            ..Default::default()
        }),
    })
}

pub fn build_incrby_command(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Err("ERR wrong number of arguments".to_string());
    }
    let increment = args[1]
        .parse::<i64>()
        .map_err(|_| "ERR value is not an integer".to_string())?;
    Ok(Command {
        command_type: CommandType::INCRBY,
        args: CommandArgs::SingleKey(Key::new(
            args[0].to_string(),
            increment.to_string(),
            None,
        )),
    })
}

fn build_push_command(args: &[String], cmd_type: CommandType) -> Result<Command, String> {
    Ok(Command {
        command_type: cmd_type,
        args: CommandArgs::KeyWithValues(KeyList {
            name: args[0].to_string(),
            values: args.iter().skip(1).cloned().collect::<Vec<String>>(),
        }),
    })
}

pub fn build_lpush_command(args: &[String]) -> Result<Command, String> {
    build_push_command(args, CommandType::LPUSH)
}

pub fn build_rpush_command(args: &[String]) -> Result<Command, String> {
    build_push_command(args, CommandType::RPUSH)
}

pub fn build_lrange_command(args: &[String]) -> Result<Command, String> {
    if args.len() < 3 {
        return Err("ERR wrong number of arguments".to_string());
    }

    Ok(Command {
        command_type: CommandType::LRANGE,
        args: CommandArgs::KeyWithValues(KeyList {
            name: args[0].to_string(),
            values: vec![args[1].to_string(), args[2].to_string()],
        }),
    })
}

pub fn build_lpop_command(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err("ERR wrong number of arguments".to_string());
    }
    Ok(Command {
        command_type: CommandType::LPOP,
        args: CommandArgs::SingleKey(Key {
            name: args[0].to_string(),
            value: args.get(1).cloned(),
            ..Default::default()
        }),
    })
}

pub fn build_rpop_command(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err("ERR wrong number of arguments".to_string());
    }
    Ok(Command {
        command_type: CommandType::RPOP,
        args: CommandArgs::SingleKey(Key {
            name: args[0].to_string(),
            value: args.get(1).cloned(),
            ..Default::default()
        }),
    })
}

pub fn build_hset_command(args: &[String]) -> Result<Command, String> {
    if args.len() < 3 || args.len() % 2 == 0 {
        return Err("ERR wrong number of arguments".to_string());
    }

    let key_name = args[0].clone();
    let fields = args[1..]
        .iter()
        .step_by(2) // Selects every other element starting from the first (field)
        .zip(args[2..].iter().step_by(2)) // Pairs each field with the next value. zip() is great!
        .map(|(field, value)| (field.clone(), value.clone())) // Convert to owned Strings
        .collect::<IndexMap<String, String>>();

    Ok(Command {
        command_type: CommandType::HSET,
        args: CommandArgs::HashKey(KeyHash {
            name: key_name,
            fields,
        }),
    })
}

pub fn build_hget_command(args: &[String]) -> Result<Command, String> {
    if args.len() != 2 {
        return Err("ERR wrong number of arguments".to_string());
    }

    Ok(Command {
        command_type: CommandType::HGET,
        args: CommandArgs::HashField(HashField {
            key: args[0].clone(),
            field: args[1].clone(),
        }),
    })
}
