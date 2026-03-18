use crate::entities::{guilds, invites};
use deadpool_redis::Pool;
use deadpool_redis::redis::cmd;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::time::Duration;

pub fn start_janitor(db: DatabaseConnection, redis: Pool) {
    tokio::spawn(async move {
        println!("Background janitor started.");

        loop {
            tokio::time::sleep(Duration::from_secs(3600)).await;

            cleanup_expired_invites(&db).await;
            cleanup_stale_presence(&db, &redis).await;
        }
    });
}

async fn cleanup_expired_invites(db: &DatabaseConnection) {
    let now = chrono::Utc::now().naive_utc();

    match invites::Entity::delete_many()
        .filter(invites::Column::ExpiresAt.lt(now))
        .exec(db)
        .await
    {
        Ok(res) if res.rows_affected > 0 => {
            println!("Janitor cleaned up {} expired invites.", res.rows_affected);
        }
        Err(e) => eprintln!("Janitor failed to clean invites: {}", e),
        _ => {}
    }
}

async fn cleanup_stale_presence(db: &DatabaseConnection, redis: &Pool) {
    let mut conn = match redis.get().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Janitor failed to get Redis connection: {}", e);
            return;
        }
    };

    let keys: Vec<String> = match cmd("KEYS")
        .arg("guild:*:members")
        .query_async(&mut conn)
        .await
    {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Janitor failed to list guild presence keys: {}", e);
            return;
        }
    };

    let mut removed = 0;

    for key in keys {
        let parts: Vec<&str> = key.split(':').collect();
        if parts.len() != 3 {
            continue;
        }
        let guild_id = parts[1];

        let exists = guilds::Entity::find_by_id(guild_id)
            .one(db)
            .await
            .unwrap_or(None)
            .is_some();

        if !exists {
            let _: Result<(), _> = cmd("DEL").arg(&key).query_async(&mut conn).await;
            removed += 1;
        }
    }

    if removed > 0 {
        println!("Janitor removed {} stale guild presence sets.", removed);
    }
}
