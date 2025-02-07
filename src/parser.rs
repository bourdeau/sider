use crate::aof::write_aof;
use crate::command::{Command, CommandType, Key};

pub async fn parse_command(command: &str, restore: bool) -> Result<Command, String> {
    let parts: Vec<&str> = command.split_whitespace().collect();

    let command_type = match parts[0].to_uppercase().as_str() {
        "PING" => CommandType::PONG,
        "GET" => CommandType::GET,
        "SET" => CommandType::SET,
        "DEL" => CommandType::DELETE,
        "FLUSHDB" => CommandType::FLUSHDB,
        "KEYS" => CommandType::KEYS,
        "EXISTS" => CommandType::EXISTS,
        "EXPIRE" => CommandType::EXPIRE,
        "TTL" => CommandType::TTL,
        "INCR" => CommandType::INCR,
        "DECR" => CommandType::DECR,
        _ => return Err(format!("Unknown command: {}", parts[0])),
    };

    let keys = parts
        .iter()
        .skip(1)
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();

    let error = Err("ERR wrong number of arguments for command".to_string());

    if command_type == CommandType::SET && keys.len() < 2 {
        return error;
    }
    if command_type == CommandType::DELETE && keys.is_empty() {
        return error;
    }
    if command_type == CommandType::EXPIRE && keys.len() < 2 {
        return error;
    }
    if command_type == CommandType::TTL && keys.is_empty() {
        return error;
    }

    let key_objects = match command_type {
        CommandType::PONG | CommandType::FLUSHDB => vec![],
        CommandType::SET => {
            vec![Key::new(keys[0].clone(), keys[1].clone(), None)]
        }
        CommandType::GET
        | CommandType::KEYS
        | CommandType::TTL
        | CommandType::INCR
        | CommandType::DECR => {
            vec![Key {
                name: keys[0].clone(),
                ..Default::default()
            }]
        }
        CommandType::DELETE | CommandType::EXISTS => keys
            .iter()
            .map(|key| Key {
                name: key.clone(),
                ..Default::default()
            })
            .collect(),
        CommandType::EXPIRE => {
            let ttl = keys[1]
                .parse::<i64>()
                .map_err(|_| "ERR value is not an integer".to_string())?;
            vec![Key {
                name: keys[0].clone(),
                expires_at: Some(ttl),
                ..Default::default()
            }]
        }
    };

    let command = Command {
        command_type,
        keys: key_objects,
    };

    if !restore {
        write_aof(&command)
            .await
            .expect("Error writing to AOF file!");
    }

    Ok(command)
}
