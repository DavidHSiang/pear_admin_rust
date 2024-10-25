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
    pub name: String,
    pub code: String,
    pub enable: i32,
    pub remark: Option<String>,
    pub details: Option<String>,
    pub sort: i32,
    pub create_at: String,
    pub update_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    Code,
    Enable,
    Remark,
    Details,
    Sort,
    CreateAt,
    UpdateAt,
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
pub enum Relation {
    RolePower,
    UserRole,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Name => ColumnType::String(StringLen::N(255u32)).def(),
            Self::Code => ColumnType::String(StringLen::N(255u32)).def(),
            Self::Enable => ColumnType::Integer.def(),
            Self::Remark => ColumnType::String(StringLen::N(255u32)).def().null(),
            Self::Details => ColumnType::String(StringLen::N(255u32)).def().null(),
            Self::Sort => ColumnType::Integer.def(),
            Self::CreateAt => ColumnType::custom("DATETIME").def(),
            Self::UpdateAt => ColumnType::custom("DATETIME").def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::RolePower => Entity::has_many(super::role_power::Entity).into(),
            Self::UserRole => Entity::has_many(super::user_role::Entity).into(),
        }
    }
}

impl Related<super::role_power::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RolePower.def()
    }
}

impl Related<super::user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
