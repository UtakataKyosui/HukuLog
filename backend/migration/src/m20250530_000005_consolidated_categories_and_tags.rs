use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Create categories table (assumed to exist already)
        // We'll add the parent reference (from m20250527_015826_add_parent_ref_to_categories.rs)
        add_reference(m, "categories", "parent", "categories").await?;
        
        // Add tag category reference (from m20250527_020005_add_tag_category_ref_to_tags.rs)
        add_reference(m, "tags", "tag_category", "").await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Remove references
        remove_reference(m, "tags", "tag_category", "").await?;
        remove_reference(m, "categories", "parent", "categories").await?;
        
        Ok(())
    }
}