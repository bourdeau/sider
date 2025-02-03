#[derive(Debug)]
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
    pub key: String,
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

    let key = parts.get(1).map_or("", |&x| x).to_string();
    let value = parts.get(2).map_or("", |&x| x).to_string();

    Ok(Command {
        command_type,
        key,
        value: Some(value),
    })
}
