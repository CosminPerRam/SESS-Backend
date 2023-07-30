use sea_orm_migration::prelude::*;
use crate::sea_orm::DeriveIden;

use crate::m2023_07_30t20_40_56_create_filters_table::Filters;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Query::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Query::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Query::Ip).string().not_null())
                    .col(ColumnDef::new(Query::FilterId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FilterId")
                            .from(Query::Table, Query::FilterId)
                            .to(Filters::Table, Filters::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Query::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Query {
    Table,
    Id,

    Ip,
    FilterId,
}
