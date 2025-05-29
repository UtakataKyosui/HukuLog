use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Remove magic_link_token column
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .drop_column(Users::MagicLinkToken)
                .to_owned(),
        )
        .await?;

        // Remove magic_link_expiration column
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .drop_column(Users::MagicLinkExpiration)
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Add magic_link_token column back
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .add_column(ColumnDef::new(Users::MagicLinkToken).string().null())
                .to_owned(),
        )
        .await?;

        // Add magic_link_expiration column back
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .add_column(ColumnDef::new(Users::MagicLinkExpiration).timestamp_with_time_zone().null())
                .to_owned(),
        )
        .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    MagicLinkToken,
    MagicLinkExpiration,
}

