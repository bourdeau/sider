#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use sider::commands::hashsets::*;
    use sider::types::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    async fn setup_db() -> Db {
        Arc::new(RwLock::new(IndexMap::new()))
    }

    #[tokio::test]
    async fn test_hset_new_hash() {
        let db = setup_db().await;
        let fields = IndexMap::from([
            ("name".to_string(), "Smith".to_string()),
            ("first_name".to_string(), "John".to_string()),
        ]);

        let command = Command {
            command_type: CommandType::HSET,
            args: CommandArgs::HashFields {
                key: "user:1".to_string(),
                fields,
            },
        };

        let result = hset(&db, command).await.unwrap().to_string();
        assert_eq!(result, "(integer) 2\n");

        let db_read = db.read().await;
        assert!(db_read.contains_key("user:1"));
    }

    #[tokio::test]
    async fn test_hset_add_new_fields() {
        let db = setup_db().await;

        let fields = IndexMap::from([("name".to_string(), "Doe".to_string())]);
        let command = Command {
            command_type: CommandType::HSET,
            args: CommandArgs::HashFields {
                key: "user:2".to_string(),
                fields,
            },
        };

        let result = hset(&db, command).await.unwrap().to_string();
        assert_eq!(result, "(integer) 1\n");

        let fields = IndexMap::from([("age".to_string(), "30".to_string())]);
        let command = Command {
            command_type: CommandType::HSET,
            args: CommandArgs::HashFields {
                key: "user:2".to_string(),
                fields,
            },
        };

        let result = hset(&db, command).await.unwrap().to_string();
        assert_eq!(result, "(integer) 1\n");

        let db_read = db.read().await;
        let stored_hash = match db_read.get("user:2") {
            Some(DbValue::HashKey(hash)) => hash,
            _ => panic!("Expected HashKey"),
        };

        assert_eq!(stored_hash.fields.get("name"), Some(&"Doe".to_string()));
        assert_eq!(stored_hash.fields.get("age"), Some(&"30".to_string()));
    }

    #[tokio::test]
    async fn test_hdel() {
        let db = setup_db().await;

        let fields = IndexMap::from([
            ("last_name".to_string(), "Smith".to_string()),
            ("first_name".to_string(), "John".to_string()),
            ("age".to_string(), "21".to_string()),
        ]);

        let command = Command {
            command_type: CommandType::HSET,
            args: CommandArgs::HashFields {
                key: "hdelhash".to_string(),
                fields,
            },
        };

        let result = hset(&db, command).await.unwrap().to_string();
        assert_eq!(result, "(integer) 3\n");

        let command = Command {
            command_type: CommandType::HDEL,
            args: CommandArgs::KeyWithValues {
                key: "hdelhash".to_string(),
                values: vec!["last_name".to_string(), "first_name".to_string()],
            },
        };

        let result = hdel(&db, command).await.unwrap().to_string();
        assert!(result.contains("(integer) 2"));

        let command = Command {
            command_type: CommandType::HDEL,
            args: CommandArgs::KeyWithValues {
                key: "hdelhash".to_string(),
                values: vec!["non_existent_field".to_string()],
            },
        };

        let result = hdel(&db, command).await.unwrap().to_string();
        assert_eq!(result, "(integer) 0\n");

        let command = Command {
            command_type: CommandType::HDEL,
            args: CommandArgs::KeyWithValues {
                key: "unknownhash".to_string(),
                values: vec!["some_field".to_string()],
            },
        };

        let result = hdel(&db, command).await.unwrap().to_string();
        assert_eq!(result, "(integer) 0\n");

        let command = Command {
            command_type: CommandType::HDEL,
            args: CommandArgs::KeyWithValues {
                key: "hdelhash".to_string(),
                values: vec!["age".to_string()],
            },
        };

        let result = hdel(&db, command).await.unwrap().to_string();
        assert!(result.contains("(integer) 1"));

        let db_read = db.read().await;
        assert!(!db_read.contains_key("hdelhash"));
    }
}
