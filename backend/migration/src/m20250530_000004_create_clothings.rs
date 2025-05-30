use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Clothings::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Clothings::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Clothings::Pid).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Clothings::Name).string().not_null())
                    .col(ColumnDef::new(Clothings::Color).string().null())
                    .col(ColumnDef::new(Clothings::Size).string().null())
                    .col(ColumnDef::new(Clothings::Material).string().null())
                    .col(ColumnDef::new(Clothings::PurchasedAt).date().null())
                    .col(ColumnDef::new(Clothings::PurchasePrice).decimal().null())
                    .col(ColumnDef::new(Clothings::WearCount).integer().default(0))
                    .col(ColumnDef::new(Clothings::LastWornAt).date().null())
                    .col(ColumnDef::new(Clothings::Season).string().null())
                    .col(ColumnDef::new(Clothings::Condition).string().null())
                    .col(ColumnDef::new(Clothings::ImageUrl).string().null())
                    .col(ColumnDef::new(Clothings::Notes).text().null())
                    .col(ColumnDef::new(Clothings::UserId).uuid().not_null())
                    .col(ColumnDef::new(Clothings::BrandId).uuid().null())
                    .col(ColumnDef::new(Clothings::CategoryId).uuid().null())
                    .col(ColumnDef::new(Clothings::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Clothings::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_clothings_user_id")
                            .from(Clothings::Table, Clothings::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_clothings_brand_id")
                            .from(Clothings::Table, Clothings::BrandId)
                            .to(Brands::Table, Brands::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_clothings_category_id")
                            .from(Clothings::Table, Clothings::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Clothings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Clothings {
    Table,
    Id,
    Pid,
    Name,
    Color,
    Size,
    Material,
    PurchasedAt,
    PurchasePrice,
    WearCount,
    LastWornAt,
    Season,
    Condition,
    ImageUrl,
    Notes,
    UserId,
    BrandId,
    CategoryId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Brands {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Categories {
    Table,
    Id,
}