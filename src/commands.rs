use crate::types::{Command, CommandArgs, CommandType, Key};

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
        command_type: CommandType::DELETE,
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
