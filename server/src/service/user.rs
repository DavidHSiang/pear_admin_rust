use crate::auto_service;
use crate::domain::models::admin;
use crate::utils::auto_service::prelude::*;
use query_filters_macros::QueryFilters;

auto_service!(AdminService, admin);

#[derive(Debug, Default, QueryFilters)]
pub struct AdminSearchParams {
    #[filter(with = admin::Column::LoginName.contains)]
    pub login_name: Option<String>,
    #[filter(with = admin::Column::Phone.eq)]
    pub phone: Option<String>,
    #[filter(with = admin::Column::Email.contains)]
    pub email: Option<String>,
}

impl AdminService {
    pub async fn page<M>(
        txn: &DatabaseTransaction,
        params: impl TryInto<PageParams, Error: Into<anyhow::Error>>
            + TryInto<AdminSearchParams, Error: Into<anyhow::Error>>
            + Clone,
    ) -> Result<PageData<M>>
    where
        M: FromQueryResult + Send + Sync,
    {
        Self::_page::<M, AdminSearchParams>(txn, params.clone(), params).await
    }
}

// #[cfg(test)]
// mod tests {
//     use std::env;

//     use super::*;
//     use crate::db;
//     use crate::domain::models::admin::ActiveModel;
//     use anyhow::Result;
//     use sea_orm::ActiveValue::NotSet;
//     use sea_orm::{Set, TransactionTrait};
//     use serde::Deserialize;

//     #[derive(Debug, Clone, Deserialize)]
//     pub struct AdminPageDTO {
//         page: Option<u64>,
//         limit: Option<u64>,
//         login_name: Option<String>,
//         phone: Option<String>,
//         email: Option<String>,
//     }

//     impl TryFrom<AdminPageDTO> for AdminSearchParams {
//         type Error = anyhow::Error;

//         fn try_from(value: AdminPageDTO) -> Result<Self, Self::Error> {
//             Ok(Self {
//                 login_name: value.login_name.filter(|data| !data.is_empty()),
//                 phone: value.phone.filter(|data| !data.is_empty()),
//                 email: value.email.filter(|data| !data.is_empty()),
//             })
//         }
//     }

//     impl TryFrom<AdminPageDTO> for PageParams {
//         type Error = anyhow::Error;

//         fn try_from(value: AdminPageDTO) -> Result<Self, Self::Error> {
//             Ok(Self {
//                 page_num: value.page.unwrap_or(1),
//                 page_size: value.limit.unwrap_or(10),
//             })
//         }
//     }

//     #[tokio::test]
//     async fn test_page() -> Result<()> {
//         let root_path = env::current_dir().unwrap().join("../");
//         assert!(env::set_current_dir(&root_path).is_ok());
//         let user_page_dto = AdminPageDTO {
//             page: Some(1),
//             limit: Some(10),
//             login_name: Some("test".to_string()),
//             phone: None,
//             email: None,
//         };
//         let txn = db!().begin().await.unwrap();
//         let res = AdminService::page::<admin::Model>(&txn, user_page_dto).await?;
//         txn.commit().await.unwrap();
//         println!("{:?}", res);
//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_get_by_id() -> Result<()> {
//         let root_path = env::current_dir().unwrap().join("../");
//         assert!(env::set_current_dir(&root_path).is_ok());
//         let txn = db!().begin().await.unwrap();
//         let res = AdminService::get_by_id::<admin::Model>(&txn, 1).await?;
//         txn.commit().await.unwrap();
//         println!("{:?}", res);
//         Ok(())
//     }

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
// }
