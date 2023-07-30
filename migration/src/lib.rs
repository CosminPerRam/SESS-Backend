pub use sea_orm_migration::prelude::*;

pub(crate) mod m2023_07_30t20_40_56_create_filters_table;
pub(crate) mod m2023_07_30t21_15_30_create_query_table;

pub mod entities;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m2023_07_30t20_40_56_create_filters_table::Migration),
            Box::new(m2023_07_30t21_15_30_create_query_table::Migration)
        ]
    }
}
