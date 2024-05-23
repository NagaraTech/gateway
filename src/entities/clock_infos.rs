//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "clock_infos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub clock: String,
    pub node_id: String,
    pub message_id: String,
    pub clock_hash: String,
    pub event_count: i32,
    pub create_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
