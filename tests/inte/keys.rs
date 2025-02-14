#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use sider::commands::keys::*;
    use sider::types::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    async fn setup_db() -> Db {
        Arc::new(RwLock::new(IndexMap::new()))
    }

    #[tokio::test]
    async fn test_set_key() {
        let db = setup_db().await;
        let key = Key {
            name: "my_key".to_string(),
            value: Some("value".to_string()),
            expires_at: None,
        };
        let command = Command {
            command_type: CommandType::SET,
            args: CommandArgs::SingleKey(key),
        };

        let result = set_key(&db, command).await;
        assert_eq!(result, "OK\n");

        let db_read = db.read().await;
        assert!(db_read.contains_key("my_key"));
    }

    #[tokio::test]
    async fn test_delete_key() {
        let db = setup_db().await;
        let key_name = "key_to_delete".to_string();

        {
            let mut db_write = db.write().await;
            db_write.insert(
                key_name.clone(),
                DbValue::StringKey(Key {
                    name: key_name.clone(),
                    value: Some("value".to_string()),
                    expires_at: None,
                }),
            );
        }

        let command = Command {
            command_type: CommandType::DEL,
            args: CommandArgs::SingleKey(Key {
                name: key_name.clone(),
                value: None,
                expires_at: None,
            }),
        };

        let result = delete_key(&db, command).await;
        assert_eq!(result, "(integer) 1\n");

        let db_read = db.read().await;
        assert!(!db_read.contains_key(&key_name));
    }

    #[tokio::test]
    async fn test_incr_new_key() {
        let db = setup_db().await;
        let command = Command {
            command_type: CommandType::INCR,
            args: CommandArgs::SingleKey(Key {
                name: "counter".to_string(),
                value: None,
                expires_at: None,
            }),
        };

        let result = incr(&db, command).await;
        assert_eq!(result, "(integer) 1\n");
    }

    #[tokio::test]
    async fn test_incr_existing_key() {
        let db = setup_db().await;
        let key_name = "counter".to_string();

        {
            let mut db_write = db.write().await;
            db_write.insert(
                key_name.clone(),
                DbValue::StringKey(Key {
                    name: key_name.clone(),
                    value: Some("5".to_string()),
                    expires_at: None,
                }),
            );
        }

        let command = Command {
            command_type: CommandType::INCR,
            args: CommandArgs::SingleKey(Key {
                name: key_name,
                value: None,
                expires_at: None,
            }),
        };

        let result = incr(&db, command).await;
        assert_eq!(result, "(integer) 6\n");
    }

    #[tokio::test]
    async fn test_decr_new_key() {
        let db = setup_db().await;
        let command = Command {
            command_type: CommandType::DECR,
            args: CommandArgs::SingleKey(Key {
                name: "counter".to_string(),
                value: None,
                expires_at: None,
            }),
        };

        let result = decr(&db, command).await;
        assert_eq!(result, "(integer) -1\n");
    }

    #[tokio::test]
    async fn test_incrby() {
        let db = setup_db().await;
        let key_name = "counter".to_string();

        {
            let mut db_write = db.write().await;
            db_write.insert(
                key_name.clone(),
                DbValue::StringKey(Key {
                    name: key_name.clone(),
                    value: Some("10".to_string()),
                    expires_at: None,
                }),
            );
        }

        let command = Command {
            command_type: CommandType::INCRBY,
            args: CommandArgs::SingleKey(Key {
                name: key_name,
                value: Some("5".to_string()),
                expires_at: None,
            }),
        };

        let result = incrby(&db, command).await;
        assert_eq!(result, "(integer) 15\n");
    }

    #[tokio::test]
    async fn test_get_keys() {
        let db = setup_db().await;

        {
            let mut db_write = db.write().await;
            db_write.insert(
                "foo".to_string(),
                DbValue::StringKey(Key {
                    name: "foo".to_string(),
                    value: Some("bar".to_string()),
                    expires_at: None,
                }),
            );
            db_write.insert(
                "foobar".to_string(),
                DbValue::StringKey(Key {
                    name: "foobar".to_string(),
                    value: Some("baz".to_string()),
                    expires_at: None,
                }),
            );
        }

        let command = Command {
            command_type: CommandType::KEYS,
            args: CommandArgs::SingleKey(Key {
                name: "foo*".to_string(),
                value: None,
                expires_at: None,
            }),
        };

        let result = get_keys(&db, command).await;
        assert_eq!(result, "1) \"foo\"\n2) \"foobar\"\n");
    }

    #[tokio::test]
    async fn test_exists() {
        let db = setup_db().await;

        {
            let mut db_write = db.write().await;
            db_write.insert(
                "key1".to_string(),
                DbValue::StringKey(Key {
                    name: "key1".to_string(),
                    value: Some("val1".to_string()),
                    expires_at: None,
                }),
            );
        }

        let command = Command {
            command_type: CommandType::EXISTS,
            args: CommandArgs::MultipleKeys(vec![
                Key {
                    name: "key1".to_string(),
                    value: None,
                    expires_at: None,
                },
                Key {
                    name: "key2".to_string(),
                    value: None,
                    expires_at: None,
                },
            ]),
        };

        let result = exists(&db, command).await;
        assert_eq!(result, "1\n");
    }

    #[tokio::test]
    async fn test_expire() {
        let db = setup_db().await;
        let key_name = "temp_key".to_string();

        {
            let mut db_write = db.write().await;
            db_write.insert(
                key_name.clone(),
                DbValue::StringKey(Key {
                    name: key_name.clone(),
                    value: Some("value".to_string()),
                    expires_at: None,
                }),
            );
        }

        let command = Command {
            command_type: CommandType::EXPIRE,
            args: CommandArgs::SingleKey(Key {
                name: key_name.clone(),
                value: None,
                expires_at: Some(1000),
            }),
        };

        let result = expire(&db, command).await;
        assert_eq!(result, "(integer) 1\n");
    }

    #[tokio::test]
    async fn test_ttl() {
        let db = setup_db().await;
        let key_name = "temp_key".to_string();

        {
            let mut db_write = db.write().await;
            db_write.insert(
                key_name.clone(),
                DbValue::StringKey(Key {
                    name: key_name.clone(),
                    value: Some("value".to_string()),
                    expires_at: Some(5000),
                }),
            );
        }

        let command = Command {
            command_type: CommandType::TTL,
            args: CommandArgs::SingleKey(Key {
                name: key_name,
                value: None,
                expires_at: None,
            }),
        };

        let result = ttl(&db, command).await;
        assert!(result.starts_with("(integer) "));
    }

    #[test]
    fn test_convert_redis_pattern_to_regex() {
        let pattern = "foo*";
        let regex = convert_redis_pattern_to_regex(pattern);
        assert_eq!(regex, "^foo.*$");

        let pattern = "bar?";
        let regex = convert_redis_pattern_to_regex(pattern);
        assert_eq!(regex, "^bar.$");

        let pattern = "[abc]";
        let regex = convert_redis_pattern_to_regex(pattern);
        assert_eq!(regex, "^[abc]$");
    }

    #[tokio::test]
    async fn test_delete_expired_key() {
        let db = setup_db().await;
        let key_name = "expired_key".to_string();

        {
            let mut db_write = db.write().await;
            db_write.insert(
                key_name.clone(),
                DbValue::StringKey(Key {
                    name: key_name.clone(),
                    value: Some("value".to_string()),
                    expires_at: Some(0),
                }),
            );
        }

        let key = Key {
            name: key_name.clone(),
            value: Some("value".to_string()),
            expires_at: Some(0),
        };
        let result = delete_expired_key(&db, key).await;
        assert!(result);

        let db_read = db.read().await;
        assert!(!db_read.contains_key(&key_name));
    }
}
