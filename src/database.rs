use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::command::Key;

pub type Db = Arc<RwLock<HashMap<String, Key>>>;
