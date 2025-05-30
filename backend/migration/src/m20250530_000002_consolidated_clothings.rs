use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Create clothings table (from m20250528_155508_create_clothings.rs)
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

        // The following references are already included in the table creation above
        // but we'll keep them commented for reference
        
        // Add user reference (from m20250527_020141_add_user_ref_to_clothings.rs)
        // add_reference(m, "clothings", "user", "").await?;
        
        // Add brand reference (from m20250527_021848_add_brand_ref_to_clothings.rs)
        // add_reference(m, "clothings", "brand", "").await?;
        
        // Add category reference (from m20250527_022025_add_category_ref_to_clothings.rs)
        // add_reference(m, "clothings", "category", "").await?;
        
        // Add size reference (from m20250527_022156_add_size_ref_to_clothings.rs)
        // add_reference(m, "clothings", "size", "").await?;
        
        // Add condition reference (from m20250527_022325_add_condition_ref_to_clothings.rs)
        // add_reference(m, "clothings", "condition", "").await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Drop clothings table
        drop_table(m, "clothings").await?;
        
        Ok(())
    }
}