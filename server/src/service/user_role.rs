use crate::auto_service;
use crate::models::user_role;
use crate::utils::auto_service::prelude::*;

auto_service!(UserRoleService, user_role, {});

impl UserRoleService {
    pub async fn delete_by_user_id(txn: &DatabaseTransaction, user_id: i32) -> Result<()> {
        user_role::Entity::delete_many()
            .filter(user_role::Column::UserId.eq(user_id))
            .exec(txn)
            .await?;
        Ok(())
    }

    pub async fn delete_by_role_id(txn: &DatabaseTransaction, role_id: i32) -> Result<()> {
        user_role::Entity::delete_many()
            .filter(user_role::Column::RoleId.eq(role_id))
            .exec(txn)
            .await?;
        Ok(())
    }

    pub async fn add_roles_for_user(
        txn: &DatabaseTransaction,
        user_id: i32,
        role_ids: Vec<i32>,
    ) -> Result<()> {
        if role_ids.is_empty() {
            return Ok(());
        }
        let user_roles = role_ids
            .iter()
            .map(|role_id| user_role::ActiveModel {
                user_id: Set(user_id),
                role_id: Set(*role_id),
                ..Default::default()
            })
            .collect::<Vec<_>>();
        user_role::Entity::insert_many(user_roles).exec(txn).await?;
        Ok(())
    }

    pub async fn update_roles_for_user(
        txn: &DatabaseTransaction,
        user_id: i32,
        role_ids: Vec<i32>,
    ) -> Result<()> {
        Self::delete_by_user_id(txn, user_id).await?;
        Self::add_roles_for_user(txn, user_id, role_ids).await
    }
}
