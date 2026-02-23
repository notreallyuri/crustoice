use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Guilds::Table)
                    .add_column(ColumnDef::new(Guilds::BannerUrl).string().null())
                    .add_column(ColumnDef::new(Guilds::IconUrl).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Guilds::Table)
                    .drop_column(Guilds::BannerUrl)
                    .drop_column(Guilds::IconUrl)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Guilds {
    Table,
    BannerUrl,
    IconUrl,
}
