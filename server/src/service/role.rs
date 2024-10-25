use query_filters_macros::QueryFilters;
use sea_orm::ActiveValue::NotSet;
use sea_orm::Set;
use serde::Deserialize;
use validator::Validate;

use crate::auto_service;
use crate::models::{role, user_role};
use crate::utils::auto_service::prelude::*;

use super::user_role::UserRoleService;

auto_service!(RoleService, role, {});

#[derive(Debug, Default, QueryFilters, Deserialize)]
pub struct RoleSearchParams {
    #[filter(with = role::Column::Name.contains)]
    pub name: Option<String>,
    #[filter(with = role::Column::Code.contains)]
    pub code: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct MergeRoleModel {
    id: Option<i32>,
    #[validate(length(min = 1, max = 50))]
    name: String,
    #[validate(length(min = 1, max = 50))]
    code: String,
    #[validate(range(min = 0, max = 1))]
    enable: i32,
    remark: Option<String>,
    details: Option<String>,
    sort: i32,
}

impl MergeRoleModel {
    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }
}

impl From<MergeRoleModel> for role::ActiveModel {
    fn from(model: MergeRoleModel) -> Self {
        Self {
            id: model.id.map_or_else(|| NotSet, Set),
            name: Set(model.name),
            code: Set(model.code),
            enable: Set(model.enable),
            remark: Set(model.remark),
            details: Set(model.details),
            sort: Set(model.sort),
            ..Default::default()
        }
    }
}

impl RoleService {
    pub async fn page<M>(
        txn: &DatabaseTransaction,
        page_params: impl TryInto<PageParams, Error: Into<anyhow::Error>>,
        search_params: impl TryInto<RoleSearchParams, Error: Into<anyhow::Error>>,
    ) -> Result<PageData<M>>
    where
        M: FromQueryResult + Send + Sync,
    {
        Self::_page::<M, RoleSearchParams>(txn, page_params, search_params).await
    }

    pub async fn get_role_ids_by_user_id(
        txn: &DatabaseTransaction,
        user_id: i32,
    ) -> Result<Vec<i32>> {
        let user_roles = user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(user_id))
            .all(txn)
            .await?;
        let role_ids = user_roles.iter().map(|ur| ur.role_id).collect();
        Ok(role_ids)
    }

    pub async fn delete(txn: &DatabaseTransaction, role_id: i32) -> Result<()> {
        UserRoleService::delete_by_role_id(txn, role_id).await?;
        Self::_delete_by_id(txn, role_id).await
    }

    pub async fn add(
        txn: &DatabaseTransaction,
        create_params: impl TryInto<MergeRoleModel, Error: Into<anyhow::Error>>,
    ) -> Result<()> {
        let create_role_model: MergeRoleModel = create_params.try_into().map_err(Into::into)?;
        create_role_model.validate()?;

        Self::_add(txn, create_role_model).await
    }

    pub async fn update(
        txn: &DatabaseTransaction,
        update_params: impl TryInto<MergeRoleModel, Error: Into<anyhow::Error>>,
    ) -> Result<()> {
        let update_role_model: MergeRoleModel = update_params.try_into().map_err(Into::into)?;
        update_role_model.validate()?;

        Self::_update(txn, update_role_model).await
    }

    pub async fn toggle_enable(txn: &DatabaseTransaction, role_id: i32) -> Result<()> {
        let role = Self::get_by_id::<role::Model>(txn, role_id).await?;
        let enable = if role.enable == 1 { 0 } else { 1 };
        Self::_update(
            txn,
            role::ActiveModel {
                id: Set(role_id),
                enable: Set(enable),
                ..Default::default()
            },
        )
        .await
    }
}
