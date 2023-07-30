//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "query")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub ip: String,
    pub filter_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::filters::Entity",
        from = "Column::FilterId",
        to = "super::filters::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Filters,
}

impl Related<super::filters::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Filters.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
