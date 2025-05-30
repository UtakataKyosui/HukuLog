use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "users",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::Uuid),
                ("email", ColType::StringUniq),
                ("password", ColType::String),
                ("api_key", ColType::StringUniq),
                ("name", ColType::String),
                ("reset_token", ColType::StringNull),
                ("reset_sent_at", ColType::TimestampWithTimeZoneNull),
                // Passkey認証用フィールド
                ("passkey_credentials", ColType::JsonNull),
                ("passkey_challenge", ColType::StringNull),
                ("passkey_challenge_expiration", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "users").await?;
        Ok(())
    }
}
