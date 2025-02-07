use crate::aof::write_aof;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq)]
pub enum CommandType {
    PONG,
    GET,
    SET,
    DELETE,
    FLUSHDB,
    KEYS,
    EXISTS,
    EXPIRE,
    TTL,
}
#[derive(Debug)]
pub struct Command {
    pub command_type: CommandType,
    pub keys: Vec<Key>,
}

#[derive(Debug, Clone, Default)]
pub struct Key {
    pub name: String,
    pub value: Option<String>,
    pub expires_at: Option<i64>,
}

impl Key {
    pub fn new(name: String, value: String, expires_at: Option<i64>) -> Self {
        Key {
            name,
            value: Some(value),
            expires_at,
        }
    }

    pub fn get_name_value_as_string(&self) -> String {
        match &self.value {
            Some(v) => format!("{} {}", self.name, v),
            None => self.name.to_string(),
        }
    }

    fn get_current_timestamp(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64
    }

    pub fn get_ttl(&self) -> i64 {
        let current_ts = self.get_current_timestamp();
        self.expires_at
            .map_or(-1, |expires_at| expires_at - current_ts)
    }

    pub fn set_ttl(&mut self, ttl: i64) {
        self.expires_at = Some(self.get_current_timestamp() + ttl);
    }

    // A key without ttl return -1 and is not expired
    pub fn is_expired(&self) -> bool {
        let ttl = self.get_ttl();
        ttl <= -2
    }
}

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
        CommandType::GET | CommandType::KEYS | CommandType::TTL => vec![Key {
            name: keys[0].clone(),
            ..Default::default()
        }],
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
