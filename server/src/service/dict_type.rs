use query_filters_macros::QueryFilters;
use sea_orm::ActiveValue::NotSet;
use serde::Deserialize;
use validator::Validate;

use crate::auto_service;
use crate::models::dict_type;
use crate::utils::auto_service::prelude::*;

use super::dict_data::DictDataService;

auto_service!(DictTypeService, dict_type, {});

#[derive(Debug, Default, QueryFilters, Deserialize)]
pub struct DictTypeSearchParams {
    #[filter(with = dict_type::Column::TypeName.contains)]
    pub type_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct MergeDictTypeModel {
    id: Option<i32>,
    #[validate(length(min = 1, max = 50))]
    type_name: String,
    #[validate(length(min = 1, max = 50))]
    type_code: String,
    #[validate(length(min = 0, max = 255))]
    description: Option<String>,
    #[validate(range(min = 0, max = 1))]
    enable: i32,
}

impl MergeDictTypeModel {
    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }
}

impl From<MergeDictTypeModel> for dict_type::ActiveModel {
    fn from(model: MergeDictTypeModel) -> Self {
        Self {
            id: model.id.map_or_else(|| NotSet, Set),
            type_name: Set(model.type_name),
            type_code: Set(model.type_code),
            description: Set(model.description),
            enable: Set(model.enable),
            ..Default::default()
        }
    }
}

impl DictTypeService {
    pub async fn page<M>(
        txn: &DatabaseTransaction,
        page_params: impl Into<PageParams>,
        search_params: impl Into<DictTypeSearchParams>,
    ) -> Result<PageData<M>>
    where
        M: FromQueryResult + Send + Sync,
    {
        Self::_page::<M, DictTypeSearchParams>(txn, page_params, search_params).await
    }

    pub async fn delete(txn: &DatabaseTransaction, dict_type_id: i32) -> Result<()> {
        let dict_type: dict_type::Model = Self::get_by_id(txn, dict_type_id).await?;
        DictDataService::delete_by_type_code(txn, dict_type.type_code).await?;
        Self::_delete_by_id(txn, dict_type_id).await
    }

    pub async fn add(
        txn: &DatabaseTransaction,
        create_params: impl TryInto<MergeDictTypeModel, Error: Into<anyhow::Error>>,
    ) -> Result<()> {
        let create_model: MergeDictTypeModel = create_params.try_into().map_err(Into::into)?;
        create_model.validate()?;

        Self::_add(txn, create_model).await
    }

    pub async fn update(
        txn: &DatabaseTransaction,
        update_params: impl TryInto<MergeDictTypeModel, Error: Into<anyhow::Error>>,
    ) -> Result<()> {
        let update_model: MergeDictTypeModel = update_params.try_into().map_err(Into::into)?;
        update_model.validate()?;

        Self::_update(txn, update_model).await
    }

    pub async fn toggle_enable(txn: &DatabaseTransaction, dict_type_id: i32) -> Result<()> {
        let dict_type = Self::get_by_id::<dict_type::Model>(txn, dict_type_id).await?;
        let enable = if dict_type.enable == 1 { 0 } else { 1 };
        Self::_update(
            txn,
            dict_type::ActiveModel {
                id: Set(dict_type_id),
                enable: Set(enable),
                ..Default::default()
            },
        )
        .await
    }
}
