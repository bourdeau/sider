#[derive(Debug, PartialEq)]
pub enum CommandType {
    PONG,
    GET,
    SET,
    DELETE,
    FLUSHDB,
}
#[derive(Debug)]
pub struct Command {
    pub command_type: CommandType,
    pub keys: Vec<String>,
    pub value: Option<String>,
}

pub fn parse_command(command: &str) -> Result<Command, String> {
    let parts: Vec<&str> = command.split_whitespace().collect();

    let command_type = match parts[0].to_uppercase().as_str() {
        "PING" => CommandType::PONG,
        "GET" => CommandType::GET,
        "SET" => CommandType::SET,
        "DEL" => CommandType::DELETE,
        "FLUSHDB" => CommandType::FLUSHDB,
        _ => return Err(format!("Unknown command: {}", parts[0])),
    };

    let keys = parts
        .iter()
        .skip(1)
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let value = parts.get(2).map_or("", |&x| x).to_string();

    if command_type == CommandType::SET && keys.len() < 2 {
        return Err("SET command requires at least 2 arguments".to_string());
    }
    if command_type == CommandType::DELETE && keys.is_empty() {
        return Err("DEL command requires at least 1 argument".to_string());
    }

    Ok(Command {
        command_type,
        keys,
        value: Some(value),
    })
}
