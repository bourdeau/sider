use crate::database::Db;
use crate::parser::parse_command;
use crate::types::CommandType;

use crate::operation::{
    decr, delete_key, exists, expire, flush_db, get_key, get_keys, incr, incrby, lpop, lpush,
    lrange, pong, rpush, set_key, ttl,
};

pub async fn process_command(command: String, db: &Db, restore: bool) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        return "ERROR: Empty command\n".to_string();
    }

    let command = match parse_command(&command, restore).await {
        Ok(cmd) => cmd,
        Err(e) => return format!("ERROR: {}\n", e),
    };

    match command.command_type {
        CommandType::PONG => pong().await,
        CommandType::GET => get_key(db, command).await,
        CommandType::SET => set_key(db, command).await,
        CommandType::DELETE => delete_key(db, command).await,
        CommandType::FLUSHDB => flush_db(db).await,
        CommandType::KEYS => get_keys(db, command).await,
        CommandType::EXISTS => exists(db, command).await,
        CommandType::EXPIRE => expire(db, command).await,
        CommandType::TTL => ttl(db, command).await,
        CommandType::INCR => incr(db, command).await,
        CommandType::DECR => decr(db, command).await,
        CommandType::INCRBY => incrby(db, command).await,
        CommandType::LPUSH => lpush(db, command).await,
        CommandType::LRANGE => lrange(db, command).await,
        CommandType::RPUSH => rpush(db, command).await,
        CommandType::LPOP => lpop(db, command).await,
    }
}
