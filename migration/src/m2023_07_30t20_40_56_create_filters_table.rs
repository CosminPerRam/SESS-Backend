use sea_orm_migration::prelude::*;
use crate::sea_orm::DeriveIden;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Filters::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Filters::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Filters::IsSecured).boolean().not_null())
                    .col(ColumnDef::new(Filters::RunsMap).string().not_null())
                    .col(ColumnDef::new(Filters::CanHavePassword).boolean().not_null())
                    .col(ColumnDef::new(Filters::CanBeEmpty).boolean().not_null())
                    .col(ColumnDef::new(Filters::IsEmpty).boolean().not_null())
                    .col(ColumnDef::new(Filters::CanBeFull).boolean().not_null())
                    .col(ColumnDef::new(Filters::RunsAppId).unsigned().not_null())
                    .col(ColumnDef::new(Filters::NotAppId).unsigned().not_null())
                    .col(ColumnDef::new(Filters::Tags).string().not_null())
                    .col(ColumnDef::new(Filters::MatchName).string().not_null())
                    .col(ColumnDef::new(Filters::MatchVersion).string().not_null())
                    .col(ColumnDef::new(Filters::RestrictUniqueIp).boolean().not_null())
                    .col(ColumnDef::new(Filters::OnAddress).string().not_null())
                    .col(ColumnDef::new(Filters::Whitelisted).boolean().not_null())
                    .col(ColumnDef::new(Filters::SpectatorProxy).boolean().not_null())
                    .col(ColumnDef::new(Filters::IsDedicated).boolean().not_null())
                    .col(ColumnDef::new(Filters::RunsLinux).boolean().not_null())
                    .col(ColumnDef::new(Filters::HasGameDir).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Filters::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Filters {
    Table,
    Id,

    IsSecured,
    RunsMap,
    CanHavePassword,
    CanBeEmpty,
    IsEmpty,
    CanBeFull,
    RunsAppId,
    NotAppId,
    Tags,
    MatchName,
    MatchVersion,
    RestrictUniqueIp,
    OnAddress,
    Whitelisted,
    SpectatorProxy,
    IsDedicated,
    RunsLinux,
    HasGameDir
}
