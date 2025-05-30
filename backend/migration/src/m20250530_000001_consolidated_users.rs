use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Create users table (from m20220101_000001_users.rs)
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
                ("email_verification_token", ColType::StringNull),
                (
                    "email_verification_sent_at",
                    ColType::TimestampWithTimeZoneNull,
                ),
                ("email_verified_at", ColType::TimestampWithTimeZoneNull),
                ("magic_link_token", ColType::StringNull),
                ("magic_link_expiration", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;

        // Add passkey fields to users (from m20250529_025731_add_passkey_fields_to_users.rs)
        m.alter_table(
            Table::alter()
                .table(Alias::new("users"))
                .add_column(
                    ColumnDef::new(Alias::new("passkey_challenge"))
                        .string()
                        .null(),
                )
                .add_column(
                    ColumnDef::new(Alias::new("passkey_challenge_expires_at"))
                        .timestamp_with_time_zone()
                        .null(),
                )
                .add_column(
                    ColumnDef::new(Alias::new("passkey_credential_id"))
                        .string()
                        .null(),
                )
                .add_column(
                    ColumnDef::new(Alias::new("passkey_credential_public_key"))
                        .string()
                        .null(),
                )
                .add_column(
                    ColumnDef::new(Alias::new("passkey_credential_counter"))
                        .big_integer()
                        .null(),
                )
                .add_column(
                    ColumnDef::new(Alias::new("passkey_transports"))
                        .string()
                        .null(),
                )
                .add_column(
                    ColumnDef::new(Alias::new("passkey_user_verified"))
                        .boolean()
                        .null(),
                )
                .add_column(
                    ColumnDef::new(Alias::new("passkey_backup_eligible"))
                        .boolean()
                        .null(),
                )
                .add_column(
                    ColumnDef::new(Alias::new("passkey_backup_state"))
                        .boolean()
                        .null(),
                )
                .add_column(
                    ColumnDef::new(Alias::new("passkey_device_type"))
                        .string()
                        .null(),
                )
                .to_owned(),
        )
        .await?;

        // Remove magic link fields (from m20250529_031410_remove_magic_link_fields.rs)
        m.alter_table(
            Table::alter()
                .table(Alias::new("users"))
                .drop_column(Alias::new("magic_link_token"))
                .drop_column(Alias::new("magic_link_expiration"))
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Add back magic link fields
        m.alter_table(
            Table::alter()
                .table(Alias::new("users"))
                .add_column(
                    ColumnDef::new(Alias::new("magic_link_token"))
                        .string()
                        .null(),
                )
                .add_column(
                    ColumnDef::new(Alias::new("magic_link_expiration"))
                        .timestamp_with_time_zone()
                        .null(),
                )
                .to_owned(),
        )
        .await?;

        // Remove passkey fields
        m.alter_table(
            Table::alter()
                .table(Alias::new("users"))
                .drop_column(Alias::new("passkey_challenge"))
                .drop_column(Alias::new("passkey_challenge_expires_at"))
                .drop_column(Alias::new("passkey_credential_id"))
                .drop_column(Alias::new("passkey_credential_public_key"))
                .drop_column(Alias::new("passkey_credential_counter"))
                .drop_column(Alias::new("passkey_transports"))
                .drop_column(Alias::new("passkey_user_verified"))
                .drop_column(Alias::new("passkey_backup_eligible"))
                .drop_column(Alias::new("passkey_backup_state"))
                .drop_column(Alias::new("passkey_device_type"))
                .to_owned(),
        )
        .await?;

        // Drop users table
        drop_table(m, "users").await?;
        
        Ok(())
    }
}