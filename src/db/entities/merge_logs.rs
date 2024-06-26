//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq,Serialize, Deserialize)]
#[sea_orm(table_name = "merge_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub from_id: String,
    pub to_id: String,
    pub start_count: i32,
    pub end_count: i32,
    pub s_clock_hash: String,
    pub e_clock_hash: String,
    pub merge_at: DateTime,
    pub node_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
