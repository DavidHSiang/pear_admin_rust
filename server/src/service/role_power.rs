use sea_orm::QuerySelect;

use crate::auto_service;
use crate::models::role_power;
use crate::utils::auto_service::prelude::*;

auto_service!(RolePowerService, role_power, {});

impl RolePowerService {
    pub async fn delete_by_power_id(txn: &DatabaseTransaction, power_id: i32) -> Result<()> {
        role_power::Entity::delete_many()
            .filter(role_power::Column::PowerId.eq(power_id))
            .exec(txn)
            .await?;
        Ok(())
    }

    pub async fn delete_by_role_id(txn: &DatabaseTransaction, role_id: i32) -> Result<()> {
        role_power::Entity::delete_many()
            .filter(role_power::Column::RoleId.eq(role_id))
            .exec(txn)
            .await?;
        Ok(())
    }

    pub async fn get_power_ids_by_role_id(
        txn: &DatabaseTransaction,
        role_id: i32,
    ) -> Result<Vec<i32>> {
        let power_ids = role_power::Entity::find()
            .select_only()
            .column(role_power::Column::PowerId)
            .filter(role_power::Column::RoleId.eq(role_id))
            .into_tuple::<i32>()
            .all(txn)
            .await?;
        Ok(power_ids)
    }

    pub async fn add_powers_for_role(
        txn: &DatabaseTransaction,
        role_id: i32,
        power_ids: Vec<i32>,
    ) -> Result<()> {
        if power_ids.is_empty() {
            return Ok(());
        }
        let role_powers = power_ids
            .iter()
            .map(|power_id| role_power::ActiveModel {
                role_id: Set(role_id),
                power_id: Set(*power_id),
                ..Default::default()
            })
            .collect::<Vec<_>>();
        role_power::Entity::insert_many(role_powers)
            .exec(txn)
            .await?;
        Ok(())
    }

    pub async fn update_powers_for_role(
        txn: &DatabaseTransaction,
        role_id: i32,
        power_ids: Vec<i32>,
    ) -> Result<()> {
        Self::delete_by_role_id(txn, role_id).await?;
        Self::add_powers_for_role(txn, role_id, power_ids).await
    }
}
