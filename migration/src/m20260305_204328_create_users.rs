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
                    .table(UserSettings::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserSettings::UserId).string().not_null())
                    .col(
                        ColumnDef::new(UserSettings::Locale)
                            .string()
                            .not_null()
                            .default("en-US"),
                    )
                    .col(
                        ColumnDef::new(UserSettings::DeveloperMode)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(UserSettings::NotificationsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(UserSettings::ThemeDarkMode)
                            .string()
                            .not_null()
                            .default("system"),
                    )
                    .col(
                        ColumnDef::new(UserSettings::ThemeColor)
                            .string()
                            .not_null()
                            .default("default"),
                    )
                    .col(
                        ColumnDef::new(UserSettings::ThemeRounding)
                            .string()
                            .not_null()
                            .default("default"),
                    )
                    .col(
                        ColumnDef::new(UserSettings::ThemeSpacing)
                            .string()
                            .not_null()
                            .default("default"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_settings-user_id")
                            .from(UserSettings::Table, UserSettings::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Relationships::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Relationships::UserId).string().not_null())
                    .col(ColumnDef::new(Relationships::TargetId).string().not_null())
                    .col(ColumnDef::new(Relationships::Status).integer().not_null())
                    .col(ColumnDef::new(Relationships::Since).timestamp().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-relationship")
                            .col(Relationships::UserId)
                            .col(Relationships::TargetId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-rel-user")
                            .from(Relationships::Table, Relationships::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-rel-target")
                            .from(Relationships::Table, Relationships::TargetId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserSettings::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Relationships::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
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
enum UserSettings {
    Table,
    UserId,
    Locale,
    DeveloperMode,
    NotificationsActive,
    ThemeDarkMode,
    ThemeColor,
    ThemeRounding,
    ThemeSpacing,
}

#[derive(DeriveIden)]
enum Relationships {
    Table,
    UserId,
    TargetId,
    Status,
    Since,
}
