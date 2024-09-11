//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "role"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq)]
pub struct Model {
    pub id: i32,
    pub role_name: Option<String>,
    pub detail: Option<String>,
    pub status: Option<i32>,
    pub create_id: Option<i32>,
    pub update_id: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    RoleName,
    Detail,
    Status,
    CreateId,
    UpdateId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::RoleName => ColumnType::String(StringLen::N(255u32)).def().null(),
            Self::Detail => ColumnType::String(StringLen::N(255u32)).def().null(),
            Self::Status => ColumnType::Integer.def().null(),
            Self::CreateId => ColumnType::Integer.def().null(),
            Self::UpdateId => ColumnType::Integer.def().null(),
            Self::CreatedAt => ColumnType::custom("datetime").def().null(),
            Self::UpdatedAt => ColumnType::custom("datetime").def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
