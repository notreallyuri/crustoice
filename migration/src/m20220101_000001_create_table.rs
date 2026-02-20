use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).string().primary_key())
                    .col(
                        ColumnDef::new(Users::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .col(ColumnDef::new(Users::DisplayName).string())
                    .col(ColumnDef::new(Users::AvatarUrl).string())
                    .col(ColumnDef::new(Users::Bio).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Guilds::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Guilds::Id).string().primary_key())
                    .col(ColumnDef::new(Guilds::Name).string().not_null())
                    .col(ColumnDef::new(Guilds::OwnerId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-guild-owner")
                            .from(Guilds::Table, Guilds::OwnerId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Channels::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Channels::Id).string().primary_key())
                    .col(ColumnDef::new(Channels::GuildId).string().not_null())
                    .col(ColumnDef::new(Channels::Name).string().not_null())
                    .col(ColumnDef::new(Channels::Position).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-channel-guild")
                            .from(Channels::Table, Channels::GuildId)
                            .to(Guilds::Table, Guilds::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Channels::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Guilds::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    DisplayName,
    AvatarUrl,
    Bio,
}
#[derive(DeriveIden)]
enum Guilds {
    Table,
    Id,
    Name,
    OwnerId,
}
#[derive(DeriveIden)]
enum Channels {
    Table,
    Id,
    GuildId,
    Name,
    Position,
}
