#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use sider::types::*;
    use sider::commands::db::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    async fn setup_db() -> Db {
        Arc::new(RwLock::new(IndexMap::new()))
    }

    #[tokio::test]
    async fn test_flush_db() {
        let db = setup_db().await;

        {
            let mut db_write = db.write().await;
            db_write.insert("key1".to_string(), DbValue::StringKey(Key { name: "key1".to_string(), value: Some("value1".to_string()), expires_at: None }));
            db_write.insert("key2".to_string(), DbValue::StringKey(Key { name: "key2".to_string(), value: Some("value2".to_string()), expires_at: None }));
        }

        {
            let db_read = db.read().await;
            assert!(!db_read.is_empty());
        }

        let result = flush_db(&db).await;

        {
            let db_read = db.read().await;
            assert!(db_read.is_empty());
        }

        assert_eq!(result, "OK\n");
    }
}
