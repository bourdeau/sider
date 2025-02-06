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
    pub expires_at: Option<i64>
}

impl Key {
    pub fn new(name: String, value: String, expires_at: Option<i64>) -> Self {
        Key { 
            name,
            value: Some(value), 
            expires_at
        }
    }

    fn get_current_timestamp(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64
    }

    pub fn get_ttl(&self) -> i64 {
        self.expires_at.map_or(-1, |expires_at| expires_at - self.get_current_timestamp())
    }

    pub fn set_ttl(&mut self, ttl: i64) {
        self.expires_at = Some(self.get_current_timestamp() + ttl);
    }
}

pub fn parse_command(command: &str) -> Result<Command, String> {
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

    let key_objects = match command_type {
        CommandType::PONG | CommandType::FLUSHDB => vec![],
        CommandType::SET | CommandType::EXISTS => vec![Key::new(keys[0].clone(), keys[1].clone(), None)],
        CommandType::GET | CommandType::KEYS => vec![Key {
            name: keys[0].clone(),
            ..Default::default()
        }],
        CommandType::DELETE => keys.iter().map(|key| Key {
            name: key.clone(),
            ..Default::default()
        }).collect(),
        CommandType::EXPIRE => vec![Key {
            name: keys[0].clone(),
            expires_at: Some(keys[1].parse::<i64>().unwrap()),
            ..Default::default()
        }],

    }; 

    Ok(Command {
        command_type,
        keys: key_objects,
    })
}
