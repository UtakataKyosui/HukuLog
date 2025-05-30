use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Create user preferences table (from m20250527_023830_create_user_preferences.rs)
        create_table(
            m,
            "user_preferences",
            &[
                ("id", ColType::PkUuid),
                ("user_id", ColType::IntegerNull),
                ("tag_id", ColType::UuidNull),
                ("preference_value", ColType::Integer),
            ],
            &[],
        )
        .await?;

        // The following references are already included in the table creation above
        // but we'll keep them commented for reference
        
        // Add user reference (from m20250527_024002_add_user_ref_to_user_preferences.rs)
        // add_reference(m, "user_preferences", "user", "").await?;
        
        // Add tag reference (from m20250527_024133_add_tag_ref_to_user_preferences.rs)
        // add_reference(m, "user_preferences", "tag", "").await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Drop user preferences table
        drop_table(m, "user_preferences").await?;
        
        Ok(())
    }
}