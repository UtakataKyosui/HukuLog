use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Create join table for clothings and outfits (from m20250527_022625_create_join_table_clothings_and_outfits.rs)
        // Note: This is already handled in the outfits migration with outfit_clothing table
        
        // Create join table for clothings and colors (from m20250527_022754_create_join_table_clothings_and_colors.rs)
        create_table(
            m,
            "clothing_color",
            &[
                ("id", ColType::PkAuto),
                ("clothing_id", ColType::Uuid),
                ("color_id", ColType::Uuid),
            ],
            &[],
        )
        .await?;
        
        // Create join table for clothings and materials (from m20250527_022922_create_join_table_clothings_and_materials.rs)
        create_table(
            m,
            "clothing_material",
            &[
                ("id", ColType::PkAuto),
                ("clothing_id", ColType::Uuid),
                ("material_id", ColType::Uuid),
            ],
            &[],
        )
        .await?;
        
        // Create join table for clothings and seasons (from m20250527_023055_create_join_table_clothings_and_seasons.rs)
        create_table(
            m,
            "clothing_season",
            &[
                ("id", ColType::PkAuto),
                ("clothing_id", ColType::Uuid),
                ("season_id", ColType::Uuid),
            ],
            &[],
        )
        .await?;
        
        // Create join table for clothings and tags (from m20250527_023226_create_join_table_clothings_and_tags.rs)
        create_table(
            m,
            "clothing_tag",
            &[
                ("id", ColType::PkAuto),
                ("clothing_id", ColType::Uuid),
                ("tag_id", ColType::Uuid),
            ],
            &[],
        )
        .await?;
        
        // Create join table for outfits and tags (from m20250527_023356_create_join_table_outfits_and_tags.rs)
        create_table(
            m,
            "outfit_tag",
            &[
                ("id", ColType::PkAuto),
                ("outfit_id", ColType::Uuid),
                ("tag_id", ColType::Uuid),
            ],
            &[],
        )
        .await?;
        
        // Create join table for users and clothings (from m20250527_023527_create_join_table_users_and_clothings.rs)
        create_table(
            m,
            "user_clothing",
            &[
                ("id", ColType::PkAuto),
                ("user_id", ColType::Integer),
                ("clothing_id", ColType::Uuid),
            ],
            &[],
        )
        .await?;
        
        // Create join table for users and outfits (from m20250527_023656_create_join_table_users_and_outfits.rs)
        create_table(
            m,
            "user_outfit",
            &[
                ("id", ColType::PkAuto),
                ("user_id", ColType::Integer),
                ("outfit_id", ColType::Uuid),
            ],
            &[],
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Drop all join tables
        drop_table(m, "user_outfit").await?;
        drop_table(m, "user_clothing").await?;
        drop_table(m, "outfit_tag").await?;
        drop_table(m, "clothing_tag").await?;
        drop_table(m, "clothing_season").await?;
        drop_table(m, "clothing_material").await?;
        drop_table(m, "clothing_color").await?;
        
        Ok(())
    }
}