use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Brands::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Brands::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Brands::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Brands::Description).text().null())
                    .col(ColumnDef::new(Brands::LogoUrl).string().null())
                    .col(ColumnDef::new(Brands::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Brands::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Brands::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Brands {
    Table,
    Id,
    Name,
    Description,
    LogoUrl,
    CreatedAt,
    UpdatedAt,
}