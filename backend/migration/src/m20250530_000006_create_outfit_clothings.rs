use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OutfitClothings::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(OutfitClothings::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(OutfitClothings::OutfitId).uuid().not_null())
                    .col(ColumnDef::new(OutfitClothings::ClothingId).uuid().not_null())
                    .col(ColumnDef::new(OutfitClothings::LayerOrder).integer().null())
                    .col(ColumnDef::new(OutfitClothings::CreatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_outfit_clothings_outfit_id")
                            .from(OutfitClothings::Table, OutfitClothings::OutfitId)
                            .to(Outfits::Table, Outfits::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_outfit_clothings_clothing_id")
                            .from(OutfitClothings::Table, OutfitClothings::ClothingId)
                            .to(Clothings::Table, Clothings::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_outfit_clothings_unique")
                            .table(OutfitClothings::Table)
                            .col(OutfitClothings::OutfitId)
                            .col(OutfitClothings::ClothingId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OutfitClothings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OutfitClothings {
    Table,
    Id,
    OutfitId,
    ClothingId,
    LayerOrder,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Outfits {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Clothings {
    Table,
    Id,
}