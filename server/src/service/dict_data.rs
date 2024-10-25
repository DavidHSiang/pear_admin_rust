use query_filters_macros::QueryFilters;
use sea_orm::ActiveValue::NotSet;
use serde::Deserialize;
use validator::Validate;

use crate::auto_service;
use crate::models::dict_data;
use crate::utils::auto_service::prelude::*;

auto_service!(DictDataService, dict_data, {});

#[derive(Debug, Default, QueryFilters, Deserialize)]
pub struct DictDataSearchParams {
    #[filter(with = dict_data::Column::TypeCode.eq)]
    pub type_code: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct MergeDictDataModel {
    id: Option<i32>,
    #[validate(length(min = 1, max = 50))]
    data_label: String,
    #[validate(length(min = 1, max = 50))]
    data_value: String,
    #[validate(length(min = 1, max = 50))]
    type_code: String,
    #[validate(range(min = 0, max = 1))]
    is_default: i32,
    #[validate(range(min = 0, max = 1))]
    enable: i32,
    #[validate(length(min = 0, max = 255))]
    remark: Option<String>,
}

impl MergeDictDataModel {
    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }
}

impl From<MergeDictDataModel> for dict_data::ActiveModel {
    fn from(model: MergeDictDataModel) -> Self {
        Self {
            id: model.id.map_or_else(|| NotSet, Set),
            data_label: Set(model.data_label),
            data_value: Set(model.data_value),
            type_code: Set(model.type_code),
            is_default: Set(model.is_default),
            enable: Set(model.enable),
            remark: Set(model.remark),
            ..Default::default()
        }
    }
}

impl DictDataService {
    pub async fn page<M>(
        txn: &DatabaseTransaction,
        page_params: impl Into<PageParams>,
        search_params: impl Into<DictDataSearchParams>,
    ) -> Result<PageData<M>>
    where
        M: FromQueryResult + Send + Sync,
    {
        Self::_page::<M, DictDataSearchParams>(txn, page_params, search_params).await
    }

    pub async fn delete_by_type_code(txn: &DatabaseTransaction, type_code: String) -> Result<()> {
        // TODO Self::_delete_by_column(txn, dict_data::Column::TypeCode, type_code).await
        dict_data::Entity::delete_many()
            .filter(dict_data::Column::TypeCode.eq(type_code))
            .exec(txn)
            .await?;
        Ok(())
    }

    pub async fn delete(txn: &DatabaseTransaction, id: i32) -> Result<()> {
        Self::_delete_by_id(txn, id).await
    }

    pub async fn add(
        txn: &DatabaseTransaction,
        create_params: impl TryInto<MergeDictDataModel, Error: Into<anyhow::Error>>,
    ) -> Result<()> {
        let create_model: MergeDictDataModel = create_params.try_into().map_err(Into::into)?;
        create_model.validate()?;

        // 如果是默认值，需要将其他的默认值设置为非默认
        if create_model.is_default == 1 {
            dict_data::Entity::update_many()
                .filter(dict_data::Column::TypeCode.eq(&create_model.type_code))
                .col_expr(dict_data::Column::IsDefault, Expr::value(0))
                .exec(txn)
                .await?;
        }

        Self::_add(txn, create_model).await
    }

    pub async fn update(
        txn: &DatabaseTransaction,
        update_params: impl TryInto<MergeDictDataModel, Error: Into<anyhow::Error>>,
    ) -> Result<()> {
        let update_model: MergeDictDataModel = update_params.try_into().map_err(Into::into)?;
        update_model.validate()?;

        // 如果是默认值，需要将其他的默认值设置为非默认
        if update_model.is_default == 1 {
            dict_data::Entity::update_many()
                .filter(dict_data::Column::TypeCode.eq(&update_model.type_code))
                .col_expr(dict_data::Column::IsDefault, Expr::value(0))
                .exec(txn)
                .await?;
        }

        Self::_update(txn, update_model).await
    }

    pub async fn toggle_enable(txn: &DatabaseTransaction, id: i32) -> Result<()> {
        let dict_data = Self::get_by_id::<dict_data::Model>(txn, id).await?;
        let enable = if dict_data.enable == 1 { 0 } else { 1 };
        Self::_update(
            txn,
            dict_data::ActiveModel {
                id: Set(id),
                enable: Set(enable),
                ..Default::default()
            },
        )
        .await
    }
}
