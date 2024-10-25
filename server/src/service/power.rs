use sea_orm::ActiveValue::NotSet;
use sea_orm::{Order, QuerySelect};
use serde::Deserialize;
use validator::Validate;

use crate::auto_service;
use crate::models::{power, role_power};
use crate::utils::auto_service::prelude::*;

use super::role_power::RolePowerService;

auto_service!(PowerService, power, {});

#[derive(Debug, Deserialize, Validate)]
pub struct MergePowerModel {
    id: Option<i32>,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(range(min = 0, max = 2))]
    pub r#type: i32,
    pub code: Option<String>,
    pub url: Option<String>,
    pub open_type: Option<String>,
    pub parent_id: i32,
    pub icon: Option<String>,
    pub sort: i32,
}

impl MergePowerModel {
    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }
}

impl From<MergePowerModel> for power::ActiveModel {
    fn from(model: MergePowerModel) -> Self {
        Self {
            id: model.id.map_or_else(|| NotSet, Set),
            name: Set(model.name),
            r#type: Set(model.r#type.to_string()),
            code: Set(model.code),
            url: Set(model.url),
            open_type: Set(model.open_type),
            parent_id: Set(model.parent_id),
            icon: Set(model.icon),
            sort: Set(model.sort),
            ..Default::default()
        }
    }
}

impl PowerService {
    pub async fn get_menu_by_role_ids<M>(
        txn: &DatabaseTransaction,
        role_ids: Vec<i32>,
    ) -> Result<Vec<M>>
    where
        M: FromQueryResult + Send + Sync,
    {
        let powers = power::Entity::find()
            .inner_join(role_power::Entity)
            .filter(role_power::Column::RoleId.is_in(role_ids))
            .order_by(power::Column::Sort, Order::Asc)
            .into_model::<M>()
            .all(txn)
            .await?;
        Ok(powers)
    }

    pub async fn delete(txn: &DatabaseTransaction, power_id: i32) -> Result<()> {
        let child_count = power::Entity::find()
            .select_only()
            .column(power::Column::Id)
            .filter(power::Column::ParentId.eq(power_id))
            .count(txn)
            .await?;

        if child_count > 0 {
            return Err(anyhow::anyhow!("该权限下有子权限，不能删除"));
        }

        RolePowerService::delete_by_power_id(txn, power_id).await?;
        Self::_delete_by_id(txn, power_id).await
    }

    pub async fn add(
        txn: &DatabaseTransaction,
        create_params: impl TryInto<MergePowerModel, Error: Into<anyhow::Error>>,
    ) -> Result<()> {
        let create_power_model: MergePowerModel = create_params.try_into().map_err(Into::into)?;
        create_power_model.validate()?;

        Self::_add(txn, create_power_model).await
    }

    pub async fn update(
        txn: &DatabaseTransaction,
        update_params: impl TryInto<MergePowerModel, Error: Into<anyhow::Error>>,
    ) -> Result<()> {
        let update_power_model: MergePowerModel = update_params.try_into().map_err(Into::into)?;
        update_power_model.validate()?;

        Self::_update(txn, update_power_model).await
    }

    pub async fn toggle_enable(txn: &DatabaseTransaction, power_id: i32) -> Result<()> {
        let power = Self::get_by_id::<power::Model>(txn, power_id).await?;
        let enable = if power.enable == 1 { 0 } else { 1 };
        Self::_update(
            txn,
            power::ActiveModel {
                id: Set(power_id),
                enable: Set(enable),
                ..Default::default()
            },
        )
        .await
    }
}
