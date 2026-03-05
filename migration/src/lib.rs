pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20260305_204328_create_users;
mod m20260305_204331_create_guilds;
mod m20260305_204433_create_messages;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260305_204328_create_users::Migration),
            Box::new(m20260305_204331_create_guilds::Migration),
            Box::new(m20260305_204433_create_messages::Migration),
        ]
    }
}
