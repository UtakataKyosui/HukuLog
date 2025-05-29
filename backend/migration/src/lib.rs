#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;
mod m20250528_155508_create_clothings;

mod m20250527_015826_add_parent_ref_to_categories;
mod m20250527_020005_add_tag_category_ref_to_tags;
mod m20250527_020141_add_user_ref_to_clothings;
mod m20250527_021848_add_brand_ref_to_clothings;
mod m20250527_022025_add_category_ref_to_clothings;
mod m20250527_022156_add_size_ref_to_clothings;
mod m20250527_022325_add_condition_ref_to_clothings;
mod m20250527_022455_add_user_ref_to_outfits;
mod m20250527_022625_create_join_table_clothings_and_outfits;
mod m20250527_022754_create_join_table_clothings_and_colors;
mod m20250527_022922_create_join_table_clothings_and_materials;
mod m20250527_023055_create_join_table_clothings_and_seasons;
mod m20250527_023226_create_join_table_clothings_and_tags;
mod m20250527_023356_create_join_table_outfits_and_tags;
mod m20250527_023527_create_join_table_users_and_clothings;
mod m20250527_023656_create_join_table_users_and_outfits;
mod m20250527_023830_create_user_preferences;
mod m20250527_024002_add_user_ref_to_user_preferences;
mod m20250527_024133_add_tag_ref_to_user_preferences;
mod m20250527_024308_add_user_ref_to_wear_histories;
mod m20250527_024438_add_clothing_ref_to_wear_histories;
mod m20250527_024612_add_outfit_ref_to_wear_histories;
mod m20250529_025731_add_passkey_fields_to_users;
mod m20250529_031410_remove_magic_link_fields;
mod m20250529_120000_create_outfits;
mod m20250529_120001_create_outfit_clothing;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250528_155508_create_clothings::Migration),
            Box::new(m20250529_025731_add_passkey_fields_to_users::Migration),
            // Temporarily commented out migrations that reference non-existent tables
            // Box::new(m20250527_015826_add_parent_ref_to_categories::Migration),
            // Box::new(m20250527_020005_add_tag_category_ref_to_tags::Migration),
            // Box::new(m20250527_020141_add_user_ref_to_clothings::Migration),
            // Box::new(m20250527_021848_add_brand_ref_to_clothings::Migration),
            // Box::new(m20250527_022025_add_category_ref_to_clothings::Migration),
            // Box::new(m20250527_022156_add_size_ref_to_clothings::Migration),
            // Box::new(m20250527_022325_add_condition_ref_to_clothings::Migration),
            // Box::new(m20250527_022455_add_user_ref_to_outfits::Migration),
            // Box::new(m20250527_022625_create_join_table_clothings_and_outfits::Migration),
            // Box::new(m20250527_022754_create_join_table_clothings_and_colors::Migration),
            // Box::new(m20250527_022922_create_join_table_clothings_and_materials::Migration),
            // Box::new(m20250527_023055_create_join_table_clothings_and_seasons::Migration),
            // Box::new(m20250527_023226_create_join_table_clothings_and_tags::Migration),
            // Box::new(m20250527_023356_create_join_table_outfits_and_tags::Migration),
            // Box::new(m20250527_023527_create_join_table_users_and_clothings::Migration),
            // Box::new(m20250527_023656_create_join_table_users_and_outfits::Migration),
            // Box::new(m20250527_023830_create_user_preferences::Migration),
            // Box::new(m20250527_024002_add_user_ref_to_user_preferences::Migration),
            // Box::new(m20250527_024133_add_tag_ref_to_user_preferences::Migration),
            // Box::new(m20250527_024308_add_user_ref_to_wear_histories::Migration),
            // Box::new(m20250527_024438_add_clothing_ref_to_wear_histories::Migration),
            // Box::new(m20250527_024612_add_outfit_ref_to_wear_histories::Migration),
            Box::new(m20250529_031410_remove_magic_link_fields::Migration),
            Box::new(m20250529_120000_create_outfits::Migration),
            Box::new(m20250529_120001_create_outfit_clothing::Migration),

            // inject-above (do not remove this comment)
        ]
    }
}