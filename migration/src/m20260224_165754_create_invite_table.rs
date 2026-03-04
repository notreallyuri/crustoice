use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Invites::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Invites::InviteCode).string().primary_key())
                    .col(ColumnDef::new(Invites::GuildId).string().not_null())
                    .col(ColumnDef::new(Invites::CreatorId).string().not_null())
                    .col(
                        ColumnDef::new(Invites::MaxUses)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Invites::Uses)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Invites::RequiresApproval)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Invites::ExpiresAt).timestamp().null())
                    .col(ColumnDef::new(Invites::CreatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-invite-guild")
                            .from(Invites::Table, Invites::GuildId)
                            .to(Guilds::Table, Guilds::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-invite-creator")
                            .from(Invites::Table, Invites::CreatorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Invites::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Invites {
    Table,
    InviteCode,
    GuildId,
    CreatorId,
    MaxUses,
    Uses,
    RequiresApproval,
    ExpiresAt,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Guilds {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
