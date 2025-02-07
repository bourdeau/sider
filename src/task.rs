use crate::database::Db;
use tokio::time::{self, Duration};

pub async fn delete_expired_keys(db: Db) {
    let mut interval = time::interval(Duration::from_secs(60));

    loop {
        interval.tick().await;

        let mut db_write = db.write().await;
        db_write.retain(|_, value| !value.is_expired());
        println!(">> Deleted expired keys");
    }
}
