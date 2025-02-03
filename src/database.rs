use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type Db = Arc<RwLock<HashMap<String, String>>>;
