use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Outfits::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Outfits::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Outfits::Pid).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Outfits::Name).string().not_null())
                    .col(ColumnDef::new(Outfits::Description).text().null())
                    .col(ColumnDef::new(Outfits::ImageUrl).string().null())
                    .col(ColumnDef::new(Outfits::UserId).uuid().not_null())
                    .col(ColumnDef::new(Outfits::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Outfits::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_outfits_user_id")
                            .from(Outfits::Table, Outfits::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Outfits::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Outfits {
    Table,
    Id,
    Pid,
    Name,
    Description,
    ImageUrl,
    UserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}