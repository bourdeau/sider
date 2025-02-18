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
pub struct ExpirableKey<T> {
    pub name: String,
    pub data: T,
    pub expires_at: Option<i64>,
}

pub type Key = ExpirableKey<Option<String>>;
pub type KeyList = ExpirableKey<Vec<String>>;
pub type KeyHash = ExpirableKey<IndexMap<String, String>>;

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
