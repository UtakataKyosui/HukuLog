use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Add passkey_credentials column
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .add_column(ColumnDef::new(Users::PasskeyCredentials).json().null())
                .to_owned(),
        )
        .await?;

        // Add passkey_challenge column
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .add_column(ColumnDef::new(Users::PasskeyChallenge).string().null())
                .to_owned(),
        )
        .await?;

        // Add passkey_challenge_expiration column
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .add_column(ColumnDef::new(Users::PasskeyChallengeExpiration).timestamp_with_time_zone().null())
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Remove passkey_challenge_expiration column
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .drop_column(Users::PasskeyChallengeExpiration)
                .to_owned(),
        )
        .await?;

        // Remove passkey_challenge column
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .drop_column(Users::PasskeyChallenge)
                .to_owned(),
        )
        .await?;

        // Remove passkey_credentials column
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .drop_column(Users::PasskeyCredentials)
                .to_owned(),
        )
        .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    PasskeyCredentials,
    PasskeyChallenge,
    PasskeyChallengeExpiration,
}
