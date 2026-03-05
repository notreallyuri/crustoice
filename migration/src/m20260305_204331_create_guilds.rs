use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Guilds::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Guilds::Id).string().primary_key())
                    .col(ColumnDef::new(Guilds::Name).string().not_null())
                    .col(ColumnDef::new(Guilds::OwnerId).string().not_null())
                    .col(ColumnDef::new(Guilds::BannerUrl).string())
                    .col(ColumnDef::new(Guilds::IconUrl).string())
                    .col(ColumnDef::new(Guilds::DefaultChannelId).string())
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
            .await?;

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
            .create_table(
                Table::create()
                    .table(Channels::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Channels::Id).string().primary_key())
                    .col(ColumnDef::new(Channels::GuildId).string().not_null())
                    .col(ColumnDef::new(Channels::Name).string().not_null())
                    .col(ColumnDef::new(Channels::Position).integer().not_null())
                    .col(ColumnDef::new(Channels::CategoryId).string().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-channel-guild")
                            .from(Channels::Table, Channels::GuildId)
                            .to(Guilds::Table, Guilds::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-channel-category")
                            .from(Channels::Table, Channels::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

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
            .await?;
        manager
            .drop_table(Table::drop().table(Channels::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GuildMembers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Guilds::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Guilds {
    Table,
    Id,
    Name,
    OwnerId,
    BannerUrl,
    IconUrl,
    DefaultChannelId,
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
enum Channels {
    Table,
    Id,
    GuildId,
    Name,
    Position,
    CategoryId,
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
enum Users {
    Table,
    Id,
}
