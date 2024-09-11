use anyhow::Result;
use query_filters_macros::QueryFilters;
use query_filters_traits::QueryFiltersTrait;
use sea_orm::entity::prelude::*;
use sea_orm::Database;
mod admin {
    //! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

    use sea_orm::entity::prelude::*;

    #[derive(Copy, Clone, Default, Debug, DeriveEntity)]
    pub struct Entity;

    impl EntityName for Entity {
        fn table_name(&self) -> &str {
            "admin"
        }
    }

    #[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq)]
    pub struct Model {
        pub id: i32,
        pub login_name: Option<String>,
        pub real_name: Option<String>,
        pub password: Option<String>,
        pub level: Option<i32>,
        pub role_ids: Option<String>,
        pub phone: Option<String>,
        pub email: Option<String>,
        pub avatar: Option<String>,
        pub remark: Option<String>,
        pub salt: Option<String>,
        pub last_ip: Option<String>,
        pub last_login: Option<String>,
        pub status: Option<i32>,
        pub create_id: Option<i32>,
        pub update_id: Option<i32>,
        pub created_at: Option<String>,
        pub updated_at: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
    pub enum Column {
        Id,
        LoginName,
        RealName,
        Password,
        Level,
        RoleIds,
        Phone,
        Email,
        Avatar,
        Remark,
        Salt,
        LastIp,
        LastLogin,
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
                Self::LoginName => ColumnType::String(StringLen::N(255u32)).def().null(),
                Self::RealName => ColumnType::String(StringLen::N(255u32)).def().null(),
                Self::Password => ColumnType::String(StringLen::N(255u32)).def().null(),
                Self::Level => ColumnType::Integer.def().null(),
                Self::RoleIds => ColumnType::String(StringLen::N(255u32)).def().null(),
                Self::Phone => ColumnType::String(StringLen::N(255u32)).def().null(),
                Self::Email => ColumnType::String(StringLen::N(255u32)).def().null(),
                Self::Avatar => ColumnType::String(StringLen::N(255u32)).def().null(),
                Self::Remark => ColumnType::String(StringLen::N(255u32)).def().null(),
                Self::Salt => ColumnType::String(StringLen::N(255u32)).def().null(),
                Self::LastIp => ColumnType::String(StringLen::N(255u32)).def().null(),
                Self::LastLogin => ColumnType::String(StringLen::N(255u32)).def().null(),
                Self::Status => ColumnType::Integer.def().null(),
                Self::CreateId => ColumnType::Integer.def().null(),
                Self::UpdateId => ColumnType::Integer.def().null(),
                Self::CreatedAt => ColumnType::String(StringLen::N(255u32)).def().null(),
                Self::UpdatedAt => ColumnType::String(StringLen::N(255u32)).def().null(),
            }
        }
    }

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            panic!("No RelationDef")
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

#[derive(Debug, Default, QueryFilters)]
struct SysAdminSearchParams {
    #[filter(with = admin::Column::LoginName.contains)]
    login_name: Option<String>,
    #[filter(with = admin::Column::Phone.eq)]
    phone: Option<String>,
    #[filter(with = admin::Column::Email.contains)]
    email: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = Database::connect("sqlite://pear-admin.db").await.unwrap();
    let search_params = SysAdminSearchParams {
        login_name: Some("admin".to_string()),
        // login_name: None,
        // phone: Some("13923452345".to_string()),
        phone: None,
        email: Some("qq.com".to_string()),
        // email: None,
    };
    let select = admin::Entity::find();
    let select = search_params.apply_filters(select);
    let ret = select.all(&db).await?;
    for r in &ret {
        println!("{:?}", r);
    }
    Ok(())
    // Remove the line that tries to access the non-existent value
}