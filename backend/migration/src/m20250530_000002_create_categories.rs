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
                    .col(ColumnDef::new(Categories::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Categories::Name).string().not_null())
                    .col(ColumnDef::new(Categories::Description).text().null())
                    .col(ColumnDef::new(Categories::ParentId).uuid().null())
                    .col(ColumnDef::new(Categories::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Categories::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_categories_parent_id")
                            .from(Categories::Table, Categories::ParentId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Categories {
    Table,
    Id,
    Name,
    Description,
    ParentId,
    CreatedAt,
    UpdatedAt,
}