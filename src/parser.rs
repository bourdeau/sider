use crate::aof::write_aof;
use crate::commands::{
    build_decr_command, build_delete_command, build_exists_command, build_expire_command,
    build_flush_db_command, build_get_command, build_incr_command, build_incrby_command,
    build_keys_command, build_lpush_command, build_lrange_command, build_pong_command,
    build_rpush_command, build_set_command, build_ttl_command,
};
use crate::types::Command;

pub async fn parse_command(command: &str, restore: bool) -> Result<Command, String> {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        return Err("ERR empty command".to_string());
    }

    let args = parts
        .iter()
        .skip(1)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let command = match parts[0].to_uppercase().as_str() {
        "PING" => build_pong_command(),
        "FLUSHDB" => build_flush_db_command(),
        "GET" => build_get_command(&args)?,
        "SET" => build_set_command(&args)?,
        "DEL" => build_delete_command(&args)?,
        "KEYS" => build_keys_command(&args)?,
        "EXISTS" => build_exists_command(&args)?,
        "EXPIRE" => build_expire_command(&args)?,
        "TTL" => build_ttl_command(&args)?,
        "INCR" => build_incr_command(&args)?,
        "DECR" => build_decr_command(&args)?,
        "INCRBY" => build_incrby_command(&args)?,
        "LPUSH" => build_lpush_command(&args)?,
        "RPUSH" => build_rpush_command(&args)?,
        "LRANGE" => build_lrange_command(&args)?,
        _ => return Err(format!("Unknown command: {}", parts[0])),
    };

    if !restore {
        write_aof(&command)
            .await
            .expect("Error writing to AOF file!");
    }

    Ok(command)
}
