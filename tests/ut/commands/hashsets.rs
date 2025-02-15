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
        let key = KeyHash {
            name: "user:1".to_string(),
            fields: IndexMap::from([
                ("name".to_string(), "Smith".to_string()),
                ("first_name".to_string(), "John".to_string()),
            ]),
        };
        let command = Command {
            command_type: CommandType::HSET,
            args: CommandArgs::HashKey(key),
        };

        let result = hset(&db, command).await.unwrap();
        assert_eq!(result, "(integer) 2\n");

        let db_read = db.read().await;
        assert!(db_read.contains_key("user:1"));
    }

    #[tokio::test]
    async fn test_hset_add_new_fields() {
        let db = setup_db().await;

        let key = KeyHash {
            name: "user:2".to_string(),
            fields: IndexMap::from([("name".to_string(), "Doe".to_string())]),
        };

        let command = Command {
            command_type: CommandType::HSET,
            args: CommandArgs::HashKey(key),
        };

        let result = hset(&db, command).await.unwrap();
        assert_eq!(result, "(integer) 1\n");

        let key = KeyHash {
            name: "user:2".to_string(),
            fields: IndexMap::from([("age".to_string(), "30".to_string())]),
        };

        let command = Command {
            command_type: CommandType::HSET,
            args: CommandArgs::HashKey(key),
        };

        let result = hset(&db, command).await.unwrap();
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
    async fn test_hset_update_existing_field() {
        let db = setup_db().await;

        let key = KeyHash {
            name: "user:3".to_string(),
            fields: IndexMap::from([("name".to_string(), "Doe".to_string())]),
        };

        let command = Command {
            command_type: CommandType::HSET,
            args: CommandArgs::HashKey(key),
        };

        let result = hset(&db, command).await.unwrap();
        assert_eq!(result, "(integer) 1\n");

        let key = KeyHash {
            name: "user:3".to_string(),
            fields: IndexMap::from([("name".to_string(), "Smith".to_string())]),
        };

        let command = Command {
            command_type: CommandType::HSET,
            args: CommandArgs::HashKey(key),
        };

        let result = hset(&db, command).await.unwrap();
        assert_eq!(result, "(integer) 0\n");

        let db_read = db.read().await;

        let stored_hash = match db_read.get("user:3") {
            Some(DbValue::HashKey(hash)) => hash,
            _ => panic!("Expected HashKey"),
        };

        assert_eq!(stored_hash.fields.get("name"), Some(&"Smith".to_string()));
    }

    #[tokio::test]
    async fn test_hget_existing_field() {
        let db = setup_db().await;

        let key = KeyHash {
            name: "user:1".to_string(),
            fields: IndexMap::from([
                ("name".to_string(), "Smith".to_string()),
                ("first_name".to_string(), "John".to_string()),
            ]),
        };

        let command = Command {
            command_type: CommandType::HSET,
            args: CommandArgs::HashKey(key),
        };

        let _ = hset(&db, command).await;

        let command = Command {
            command_type: CommandType::HGET,
            args: CommandArgs::HashField(HashField {
                key: "user:1".to_string(),
                field: "name".to_string(),
            }),
        };

        let result = hget(&db, command).await.unwrap();
        assert_eq!(result, "Smith\n");
    }

    #[tokio::test]
    async fn test_hget_nonexistent_field() {
        let db = setup_db().await;

        let key = KeyHash {
            name: "user:2".to_string(),
            fields: IndexMap::from([("name".to_string(), "Doe".to_string())]),
        };

        let command = Command {
            command_type: CommandType::HSET,
            args: CommandArgs::HashKey(key),
        };

        let _ = hset(&db, command).await;

        let command = Command {
            command_type: CommandType::HGET,
            args: CommandArgs::HashField(HashField {
                key: "user:2".to_string(),
                field: "age".to_string(),
            }),
        };

        let result = hget(&db, command).await.unwrap();
        assert_eq!(result, "(nil)\n");
    }

    #[tokio::test]
    async fn test_hget_nonexistent_key() {
        let db = setup_db().await;

        let command = Command {
            command_type: CommandType::HGET,
            args: CommandArgs::HashField(HashField {
                key: "user:3".to_string(),
                field: "name".to_string(),
            }),
        };

        let result = hget(&db, command).await.unwrap();
        assert_eq!(result, "(nil)\n");
    }

    #[tokio::test]
    async fn test_hgetall_existing_hash() {
        let db = setup_db().await;

        let key = KeyHash {
            name: "user:1".to_string(),
            fields: IndexMap::from([
                ("name".to_string(), "Smith".to_string()),
                ("first_name".to_string(), "John".to_string()),
                ("age".to_string(), "21".to_string()),
            ]),
        };

        let command = Command {
            command_type: CommandType::HSET,
            args: CommandArgs::HashKey(key),
        };

        let result = hset(&db, command).await.unwrap();
        assert_eq!(result, "(integer) 3\n");

        let command = Command {
            command_type: CommandType::HGETALL,
            args: CommandArgs::KeyName("user:1".to_string()),
        };

        let result = hgetall(&db, command).await.unwrap();
        assert!(result.contains("name"));
        assert!(result.contains("Smith"));
        assert!(result.contains("first_name"));
        assert!(result.contains("John"));
        assert!(result.contains("age"));
        assert!(result.contains("21"));
    }

    #[tokio::test]
    async fn test_hgetall_non_existing_hash() {
        let db = setup_db().await;

        let command = Command {
            command_type: CommandType::HGETALL,
            args: CommandArgs::KeyName("non_existing".to_string()),
        };

        let result = hgetall(&db, command).await.unwrap();
        assert_eq!(result, "(empty array)\n");
    }

    #[tokio::test]
    async fn test_hdel() {
        let db = setup_db().await;

        let key = KeyHash {
            name: "hdelhash".to_string(),
            fields: IndexMap::from([
                ("last_name".to_string(), "Smith".to_string()),
                ("first_name".to_string(), "John".to_string()),
                ("age".to_string(), "21".to_string()),
            ]),
        };

        let command = Command {
            command_type: CommandType::HSET,
            args: CommandArgs::HashKey(key),
        };

        let result = hset(&db, command).await.unwrap();
        assert_eq!(result, "(integer) 3\n");

        let command = Command {
            command_type: CommandType::HDEL,
            args: CommandArgs::KeyWithValues(KeyList {
                name: "hdelhash".to_string(),
                values: vec!["last_name".to_string(), "first_name".to_string()],
            }),
        };

        let result = hdel(&db, command).await.unwrap();
        assert!(result.contains("(integer) 2"));

        let command = Command {
            command_type: CommandType::HDEL,
            args: CommandArgs::KeyWithValues(KeyList {
                name: "hdelhash".to_string(),
                values: vec!["non_existent_field".to_string()],
            }),
        };

        let result = hdel(&db, command).await.unwrap();
        assert_eq!(result, "(integer) 0\n");

        let command = Command {
            command_type: CommandType::HDEL,
            args: CommandArgs::KeyWithValues(KeyList {
                name: "unknownhash".to_string(),
                values: vec!["some_field".to_string()],
            }),
        };

        let result = hdel(&db, command).await.unwrap();
        assert_eq!(result, "(integer) 0\n");

        let command = Command {
            command_type: CommandType::HDEL,
            args: CommandArgs::KeyWithValues(KeyList {
                name: "hdelhash".to_string(),
                values: vec!["age".to_string()],
            }),
        };

        let result = hdel(&db, command).await.unwrap();
        assert!(result.contains("(integer) 1"));

        let db_read = db.read().await;
        assert!(!db_read.contains_key("hdelhash"));
    }
}
