pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260217_234756_create_guild_members;
mod m20260218_031048_create_relationships_table;
mod m20260220_212031_create_messages_table;
mod m20260222_153059_create_categories;
mod m20260222_160429_alter_guilds;
mod m20260224_165754_create_invite_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260217_234756_create_guild_members::Migration),
            Box::new(m20260218_031048_create_relationships_table::Migration),
            Box::new(m20260220_212031_create_messages_table::Migration),
            Box::new(m20260222_153059_create_categories::Migration),
            Box::new(m20260222_160429_alter_guilds::Migration),
            Box::new(m20260224_165754_create_invite_table::Migration),
        ]
    }
}
