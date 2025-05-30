use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Create outfits table (from m20250529_120000_create_outfits.rs)
        create_table(
            m,
            "outfits",
            &[
                ("id", ColType::PkUuid),
                ("name", ColType::String),
                ("description", ColType::TextNull),
                ("image_url", ColType::StringNull),
                ("user_id", ColType::IntegerNull),
            ],
            &[],
        )
        .await?;

        // Create outfit_clothing join table (from m20250529_120001_create_outfit_clothing.rs)
        create_table(
            m,
            "outfit_clothing",
            &[
                ("id", ColType::PkAuto),
                ("outfit_id", ColType::Uuid),
                ("clothing_id", ColType::Uuid),
            ],
            &[],
        )
        .await?;

        // The user reference is already included in the outfits table creation
        // but we'll keep it commented for reference
        
        // Add user reference (from m20250527_022455_add_user_ref_to_outfits.rs)
        // add_reference(m, "outfits", "user", "").await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Drop outfit_clothing table
        drop_table(m, "outfit_clothing").await?;
        
        // Drop outfits table
        drop_table(m, "outfits").await?;
        
        Ok(())
    }
}