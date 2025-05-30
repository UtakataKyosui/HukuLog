use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tags::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tags::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Tags::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Tags::Color).string().null())
                    .col(ColumnDef::new(Tags::CategoryId).uuid().null())
                    .col(ColumnDef::new(Tags::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Tags::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tags_category_id")
                            .from(Tags::Table, Tags::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tags::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tags {
    Table,
    Id,
    Name,
    Color,
    CategoryId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Categories {
    Table,
    Id,
}