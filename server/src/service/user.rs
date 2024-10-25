use std::str::FromStr;

use super::user_role::UserRoleService;
use super::{validate_password, NO_SPACE_REGEX};
use crate::auto_service;
use crate::models::dept;
use crate::models::user;
use crate::utils::auto_service::prelude::*;
use crate::utils::rand_utils::encrypt_password;
use crate::utils::rand_utils::rand_s;
use query_filters_macros::QueryFilters;
use sea_orm::Set;
use sea_orm::{JoinType, QuerySelect};
use serde::de;
use serde::Deserialize;
use serde::Deserializer;
use std::fmt::Debug;
use validator::Validate;

auto_service!(UserService, user, {
    dept => [
        (JoinType::InnerJoin, user::Relation::Dept, dept, {
            Id => d_id,
            Enable => d_enable
        })
    ]
});

#[derive(Debug, Default, QueryFilters, Deserialize)]
pub struct UserSearchParams {
    #[filter(with = user::Column::Username.contains)]
    pub username: Option<String>,
    #[filter(with = user::Column::RealName.contains)]
    pub real_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct MergeUserModel {
    id: Option<i32>,
    #[validate(length(min = 1), regex(path = "*NO_SPACE_REGEX"))]
    username: String,
    #[validate(length(min = 1), regex(path = "*NO_SPACE_REGEX"))]
    real_name: String,
    #[validate(custom(function = "validate_password"))]
    password: Option<String>,
    dept_id: i32,
    #[serde(deserialize_with = "split_by_comma")]
    role_ids: Vec<i32>,
    #[validate(range(min = 0, max = 1))]
    enable: i32,
}

impl MergeUserModel {
    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }
}

impl From<MergeUserModel> for user::ActiveModel {
    fn from(model: MergeUserModel) -> Self {
        match model.id {
            Some(id) => Self {
                id: Set(id),
                username: Set(model.username),
                real_name: Set(model.real_name),
                dept_id: Set(model.dept_id),
                enable: Set(model.enable),
                ..Default::default()
            },
            None => {
                let salt = rand_s(4);
                let password_hash = encrypt_password(&model.password.unwrap(), &salt);

                Self {
                    username: Set(model.username),
                    real_name: Set(model.real_name),
                    dept_id: Set(model.dept_id),
                    password_hash: Set(password_hash),
                    salt: Set(salt),
                    enable: Set(model.enable),
                    ..Default::default()
                }
            }
        }
    }
}

impl UserService {
    pub async fn page_with_dept<M>(
        txn: &DatabaseTransaction,
        page_params: impl TryInto<PageParams, Error: Into<anyhow::Error>>,
        search_params: impl TryInto<UserSearchParams, Error: Into<anyhow::Error>>,
    ) -> Result<PageData<M>>
    where
        M: FromQueryResult + Send + Sync,
    {
        Self::_page_with_dept::<M, UserSearchParams>(txn, page_params, search_params).await
    }

    pub async fn add(
        txn: &DatabaseTransaction,
        create_params: impl TryInto<MergeUserModel, Error: Into<anyhow::Error>>,
    ) -> Result<()> {
        let create_user_model: MergeUserModel = create_params.try_into().map_err(Into::into)?;
        create_user_model.validate()?;
        let role_ids = create_user_model.role_ids.clone();
        let user = Self::add_and_fetch(txn, create_user_model).await?;
        UserRoleService::add_roles_for_user(txn, user.id, role_ids).await
    }

    pub async fn delete(txn: &DatabaseTransaction, user_id: i32) -> Result<()> {
        UserRoleService::delete_by_user_id(txn, user_id).await?;
        Self::_delete_by_id(txn, user_id).await
    }

    pub async fn update(
        txn: &DatabaseTransaction,
        update_params: impl TryInto<MergeUserModel, Error: Into<anyhow::Error>>,
    ) -> Result<()> {
        let update_user_model: MergeUserModel = update_params.try_into().map_err(Into::into)?;
        update_user_model.validate()?;

        let role_ids = update_user_model.role_ids.clone();
        let user = Self::_update_and_fetch(txn, update_user_model).await?;
        UserRoleService::update_roles_for_user(txn, user.id, role_ids).await
    }

    pub async fn toggle_enable(txn: &DatabaseTransaction, user_id: i32) -> Result<()> {
        let user: user::Model = Self::get_by_id(txn, user_id).await?;
        let enable = if user.enable == 1 { 0 } else { 1 };
        Self::_update(
            txn,
            user::ActiveModel {
                id: Set(user_id),
                enable: Set(enable),
                ..Default::default()
            },
        )
        .await
    }
}

// 自定义反序列化函数，名称更简洁
fn split_by_comma<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;

    if s.is_empty() {
        return Ok(vec![]);
    }

    s.split(',')
        .map(|s| i32::from_str(s.trim()).map_err(de::Error::custom)) // trim去除多余的空格
        .collect()
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use crate::db;
    //     use crate::domain::models::admin::ActiveModel;
    //     use anyhow::Result;
    //     use sea_orm::ActiveValue::NotSet;
    use sea_orm::TransactionTrait;
    use serde::Serialize;
    //     use serde::Deserialize;

    #[tokio::test]
    async fn test_page() -> Result<()> {
        let root_path = env::current_dir().unwrap().join("../");
        assert!(env::set_current_dir(&root_path).is_ok());

        let txn = db!().begin().await.unwrap();
        let page_params = PageParams {
            page_num: 1,
            page_size: 10,
        };

        let search_params = UserSearchParams {
            username: Some("admin".to_string()),
            real_name: None,
        };

        let res =
            UserService::_page::<user::Model, UserSearchParams>(&txn, page_params, search_params)
                .await?;
        txn.commit().await?;
        println!("{:?}", res);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_by_id() -> Result<()> {
        let root_path = env::current_dir().unwrap().join("../");
        assert!(env::set_current_dir(&root_path).is_ok());
        let txn = db!().begin().await.unwrap();
        let res = UserService::get_by_id::<user::Model>(&txn, 1).await?;
        txn.commit().await.unwrap();
        println!("{:?}", res);
        Ok(())
    }

    #[derive(Serialize, Debug, Clone, FromQueryResult, Default)]
    pub struct UserWithDept {
        id: i64,
        username: String,
        real_name: String,
        enable: i32,
        dept_id: i32,
        dept_name: String,
        d_id: i32,
    }

    #[tokio::test]
    async fn test_get_by_id_with_dept() -> Result<()> {
        let root_path = env::current_dir().unwrap().join("../");
        assert!(env::set_current_dir(&root_path).is_ok());
        let txn = db!().begin().await.unwrap();
        let res = UserService::get_by_id_with_dept::<UserWithDept>(&txn, 1).await?;
        txn.commit().await.unwrap();
        println!("{:?}", res);
        Ok(())
    }

    #[tokio::test]
    async fn test_page_with_dept() -> Result<()> {
        let root_path = env::current_dir().unwrap().join("../");
        assert!(env::set_current_dir(&root_path).is_ok());

        let txn = db!().begin().await.unwrap();
        let page_params = PageParams {
            page_num: 1,
            page_size: 10,
        };

        let search_params = UserSearchParams {
            username: None,
            real_name: None,
        };

        let res = UserService::_page_with_dept::<UserWithDept, UserSearchParams>(
            &txn,
            page_params,
            search_params,
        )
        .await?;
        txn.commit().await?;
        println!("{:?}", res);
        Ok(())
    }
    //     #[tokio::test]
    //     async fn test_add() -> Result<()> {
    //         let root_path = env::current_dir().unwrap().join("../");
    //         assert!(env::set_current_dir(&root_path).is_ok());
    //         let params = ActiveModel {
    //             id: NotSet,
    //             login_name: Set(Some("test".to_string())),
    //             phone: Set(Some("12311111111".to_string())),
    //             email: Set(Some("test@qq.com".to_string())),
    //             ..Default::default()
    //         };
    //         print!("{:?}", params);
    //         let txn = db!().begin().await.unwrap();
    //         let res = AdminService::add_and_fetch(&txn, params).await?;
    //         print!("{:?}", res);
    //         txn.commit().await.unwrap();
    //         Ok(())
    //     }
}
