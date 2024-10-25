use sea_orm::ActiveValue::NotSet;
use sea_orm::QuerySelect;
use serde::Deserialize;
use validator::Validate;

use crate::auto_service;
use crate::models::{dept, user};
use crate::utils::auto_service::prelude::*;

auto_service!(DeptService, dept, {});

#[derive(Debug, Deserialize, Validate)]
pub struct MergeDeptModel {
    id: Option<i32>,
    parent_id: Option<i32>,
    #[validate(length(min = 1, max = 50))]
    dept_name: String,
    sort: i32,
    #[validate(length(min = 1, max = 5))]
    leader: Option<String>,
    #[validate(length(min = 1, max = 11))]
    phone: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(range(min = 0, max = 1))]
    enable: i32,
    #[validate(length(max = 255))]
    remark: Option<String>,
    #[validate(length(max = 255))]
    address: Option<String>,
}

impl MergeDeptModel {
    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }
}

impl From<MergeDeptModel> for dept::ActiveModel {
    fn from(model: MergeDeptModel) -> Self {
        Self {
            id: model.id.map_or_else(|| NotSet, Set),
            parent_id: model.parent_id.map_or_else(|| NotSet, Set),
            dept_name: Set(model.dept_name),
            sort: Set(model.sort),
            leader: Set(model.leader),
            phone: Set(model.phone),
            email: Set(model.email),
            enable: Set(model.enable),
            remark: Set(model.remark),
            address: Set(model.address),
            ..Default::default()
        }
    }
}

impl DeptService {
    pub async fn delete(txn: &DatabaseTransaction, dept_id: i32) -> Result<()> {
        // seaorm 判断是否有user关联
        let user_count = user::Entity::find()
            .select_only()
            .column(user::Column::Id)
            .filter(user::Column::DeptId.eq(dept_id))
            .count(txn)
            .await?;

        if user_count > 0 {
            return Err(anyhow::anyhow!("该部门下有用户，不能删除"));
        }

        // 判断是否有dept关联
        let dept_count = dept::Entity::find()
            .select_only()
            .column(dept::Column::Id)
            .filter(dept::Column::ParentId.eq(dept_id))
            .count(txn)
            .await?;

        if dept_count > 0 {
            return Err(anyhow::anyhow!("该部门下有子部门，不能删除"));
        }
        Self::_delete_by_id(txn, dept_id).await
    }

    pub async fn add(
        txn: &DatabaseTransaction,
        create_params: impl TryInto<MergeDeptModel, Error: Into<anyhow::Error>>,
    ) -> Result<()> {
        let create_dept_model: MergeDeptModel = create_params.try_into().map_err(Into::into)?;
        create_dept_model.validate()?;

        Self::_add(txn, create_dept_model).await
    }

    pub async fn update(
        txn: &DatabaseTransaction,
        update_params: impl TryInto<MergeDeptModel, Error: Into<anyhow::Error>>,
    ) -> Result<()> {
        let update_dept_model: MergeDeptModel = update_params.try_into().map_err(Into::into)?;
        update_dept_model.validate()?;

        Self::_update(txn, update_dept_model).await
    }

    pub async fn toggle_enable(txn: &DatabaseTransaction, dept_id: i32) -> Result<()> {
        let dept = Self::get_by_id::<dept::Model>(txn, dept_id).await?;
        let enable = if dept.enable == 1 { 0 } else { 1 };
        Self::_update(
            txn,
            dept::ActiveModel {
                id: Set(dept_id),
                enable: Set(enable),
                ..Default::default()
            },
        )
        .await
    }
}
