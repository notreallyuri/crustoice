use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Categories::Id).string().primary_key())
                    .col(ColumnDef::new(Categories::GuildId).string().not_null())
                    .col(ColumnDef::new(Categories::Name).string().not_null())
                    .col(ColumnDef::new(Categories::Position).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-category-guild")
                            .from(Categories::Table, Categories::GuildId)
                            .to(Guilds::Table, Guilds::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Channels::Table)
                    .add_column(ColumnDef::new(Channels::CategoryId).string().null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-channel-category")
                            .from_tbl(Channels::Table)
                            .from_col(Channels::CategoryId)
                            .to_tbl(Categories::Table)
                            .to_col(Categories::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Channels::Table)
                    .drop_column(Channels::CategoryId)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Categories {
    Table,
    Id,
    GuildId,
    Name,
    Position,
}
#[derive(DeriveIden)]
enum Channels {
    Table,
    CategoryId,
}
#[derive(DeriveIden)]
enum Guilds {
    Table,
    Id,
}
