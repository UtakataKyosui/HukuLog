use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Create wear histories table (assumed to exist already)
        // We'll add the references
        
        // Add user reference (from m20250527_024308_add_user_ref_to_wear_histories.rs)
        add_reference(m, "wear_histories", "user", "").await?;
        
        // Add clothing reference (from m20250527_024438_add_clothing_ref_to_wear_histories.rs)
        add_reference(m, "wear_histories", "clothing", "").await?;
        
        // Add outfit reference (from m20250527_024612_add_outfit_ref_to_wear_histories.rs)
        add_reference(m, "wear_histories", "outfit", "").await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Remove references
        remove_reference(m, "wear_histories", "outfit", "").await?;
        remove_reference(m, "wear_histories", "clothing", "").await?;
        remove_reference(m, "wear_histories", "user", "").await?;
        
        Ok(())
    }
}