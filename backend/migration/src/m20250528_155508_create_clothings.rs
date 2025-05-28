use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "clothings",
            &[
                ("id", ColType::PkUuid),
                ("name", ColType::String),
                ("purchased_at", ColType::DateNull),
                ("purchase_price", ColType::DecimalNull),
                ("image_url", ColType::StringNull),
                ("notes", ColType::TextNull),
                ("user_id", ColType::IntegerNull),
                ("brand_id", ColType::UuidNull),
                ("category_id", ColType::UuidNull),
                ("size_id", ColType::UuidNull),
                ("condition_id", ColType::UuidNull),
            ],
            &[],
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "clothings").await?;
        Ok(())
    }
}