use crate::entities::invites;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::time::Duration;

pub fn start_invite_cleanup(db: DatabaseConnection) {
    tokio::spawn(async move {
        println!("Background janitor started. Sweeping expired invites every hour.");

        loop {
            tokio::time::sleep(Duration::from_secs(3600)).await;

            let now = chrono::Utc::now().naive_utc();

            let res = invites::Entity::delete_many()
                .filter(invites::Column::ExpiresAt.lt(now))
                .exec(&db)
                .await;

            match res {
                Ok(res) if res.rows_affected > 0 => {
                    println!(
                        "🧹 Janitor cleaned up {} expired invites.",
                        res.rows_affected
                    );
                }
                Err(e) => eprintln!("Janitor failed to clean invites: {}", e),
                _ => {}
            }
        }
    });
}
