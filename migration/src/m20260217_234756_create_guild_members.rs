use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GuildMembers::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(GuildMembers::GuildId).string().not_null())
                    .col(ColumnDef::new(GuildMembers::UserId).string().not_null())
                    .col(ColumnDef::new(GuildMembers::Nickname).string())
                    .col(ColumnDef::new(GuildMembers::Roles).json())
                    .col(
                        ColumnDef::new(GuildMembers::JoinedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-guild-member")
                            .col(GuildMembers::GuildId)
                            .col(GuildMembers::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-member-guild")
                            .from(GuildMembers::Table, GuildMembers::GuildId)
                            .to(Guilds::Table, Guilds::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-member-user")
                            .from(GuildMembers::Table, GuildMembers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GuildMembers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GuildMembers {
    Table,
    GuildId,
    UserId,
    Nickname,
    Roles,
    JoinedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Guilds {
    Table,
    Id,
}
