use indexmap::IndexMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type Db = Arc<RwLock<IndexMap<String, DbValue>>>;

#[derive(Debug, Clone)]
pub struct Command {
    pub command_type: CommandType,
    pub args: CommandArgs,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CommandType {
    PONG,
    GET,
    SET,
    DEL,
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
    LPOP,
    RPOP,
    HSET,
    HGET,
    HGETALL,
    HDEL,
}

#[derive(Debug, Clone)]
pub enum CommandArgs {
    KeyName(String),
    SingleKey(Key),
    KeyWithValues(KeyList),
    MultipleKeys(Vec<Key>),
    HashKey(KeyHash),
    HashField(HashField),
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

#[derive(Debug, Clone, Default)]
pub struct KeyHash {
    pub name: String,
    pub fields: IndexMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct HashField {
    pub key: String,
    pub field: String,
}

#[derive(Debug, Clone)]
pub enum DbValue {
    StringKey(Key),
    ListKey(KeyList),
    HashKey(KeyHash),
}

#[derive(Debug, Clone, Copy)]
pub enum ListPushType {
    LPUSH,
    RPUSH,
}

#[derive(Debug, Clone, Copy)]
pub enum PopType {
    LPOP,
    RPOP,
}
