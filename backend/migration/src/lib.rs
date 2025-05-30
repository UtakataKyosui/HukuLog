#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20250530_000001_create_brands;
mod m20250530_000002_create_categories;
mod m20250530_000003_create_tags;
mod m20250530_000004_create_clothings;
mod m20250530_000005_create_outfits;
mod m20250530_000006_create_outfit_clothings;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250530_000001_create_brands::Migration),
            Box::new(m20250530_000002_create_categories::Migration),
            Box::new(m20250530_000003_create_tags::Migration),
            Box::new(m20250530_000004_create_clothings::Migration),
            Box::new(m20250530_000005_create_outfits::Migration),
            Box::new(m20250530_000006_create_outfit_clothings::Migration),
        ]
    }
}