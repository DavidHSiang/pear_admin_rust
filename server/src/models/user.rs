//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "user"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq)]
pub struct Model {
    pub id: i32,
    pub username: String,
    pub real_name: String,
    pub avatar: Option<String>,
    pub remark: Option<String>,
    pub password_hash: String,
    pub enable: i32,
    pub dept_id: i32,
    pub create_at: String,
    pub update_at: String,
    pub salt: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Username,
    RealName,
    Avatar,
    Remark,
    PasswordHash,
    Enable,
    DeptId,
    CreateAt,
    UpdateAt,
    Salt,
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
    UserRole,
    Dept,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Username => ColumnType::String(StringLen::N(20u32)).def().unique(),
            Self::RealName => ColumnType::String(StringLen::N(20u32)).def(),
            Self::Avatar => ColumnType::String(StringLen::N(255u32)).def().null(),
            Self::Remark => ColumnType::String(StringLen::N(255u32)).def().null(),
            Self::PasswordHash => ColumnType::String(StringLen::N(128u32)).def(),
            Self::Enable => ColumnType::Integer.def(),
            Self::DeptId => ColumnType::Integer.def(),
            Self::CreateAt => ColumnType::custom("DATETIME").def(),
            Self::UpdateAt => ColumnType::custom("DATETIME").def(),
            Self::Salt => ColumnType::String(StringLen::N(4u32)).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UserRole => Entity::has_many(super::user_role::Entity).into(),
            Self::Dept => Entity::belongs_to(super::dept::Entity)
                .from(Column::DeptId)
                .to(super::dept::Column::Id)
                .into(),
        }
    }
}

impl Related<super::user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRole.def()
    }
}

impl Related<super::dept::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dept.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}