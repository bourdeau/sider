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
    NoArgs,                    // PONG, FLUSHDB
    SingleKey(String),         // GET key
    MultipleKeys(Vec<String>), // DEL key1 key2 key3
    KeyWithValue {
        key: String,
        value: String,
    }, // SET key value
    KeyWithValues {
        key: String,
        values: Vec<String>,
    },
    KeyWithTTL {
        key: String,
        ttl: i64,
    }, // EXPIRE key 10
    HashFields {
        key: String,
        fields: IndexMap<String, String>,
    }, // HSET key field1 value1 field2 value2
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
