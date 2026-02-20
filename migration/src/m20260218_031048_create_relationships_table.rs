use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
            .drop_table(Table::drop().table(Relationships::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Relationships {
    Table,
    UserId,
    TargetId,
    Status,
    Since,
}
#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
