#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

// Consolidated migrations
mod m20250530_000001_consolidated_users;
mod m20250530_000002_consolidated_clothings;
mod m20250530_000003_consolidated_outfits;
mod m20250530_000004_consolidated_join_tables;
mod m20250530_000005_consolidated_categories_and_tags;
mod m20250530_000006_consolidated_user_preferences;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250530_000001_consolidated_users::Migration),
            Box::new(m20250530_000002_consolidated_clothings::Migration),
            Box::new(m20250530_000003_consolidated_outfits::Migration),
            Box::new(m20250530_000004_consolidated_join_tables::Migration),
            Box::new(m20250530_000006_consolidated_user_preferences::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}