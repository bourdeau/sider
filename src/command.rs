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
    INCR,
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
