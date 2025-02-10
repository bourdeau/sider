use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type Db = Arc<RwLock<HashMap<String, DbValue>>>;

#[derive(Debug, Clone)]
pub enum DbValue {
    StringKey(Key),
    ListKey(KeyList),
}

#[derive(Debug)]
pub enum CommandArgs {
    SingleKey(Key),
    KeyWithValues(KeyList),
    MultipleKeys(Vec<Key>),
}

#[derive(Debug)]
pub struct Command {
    pub command_type: CommandType,
    pub args: CommandArgs,
}

#[derive(Debug, Clone, Default)]
pub struct Key {
    pub name: String,
    pub value: Option<String>,
    pub expires_at: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct KeyList {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
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
    INCR,
    DECR,
    INCRBY,
    LPUSH,
    LRANGE,
    RPUSH,
}

#[derive(Debug, Clone, Copy)]
pub enum ListPushType {
    LPUSH,
    RPUSH,
}
