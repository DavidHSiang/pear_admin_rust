use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use sea_orm::TransactionTrait;
use tracing::info;

use crate::{
    controller::{
        dto::PageParamsOption,
        vo::{DictTypePageVo, RespVO},
    },
    db,
    service::dict_type::{DictTypeSearchParams, DictTypeService, MergeDictTypeModel},
};

pub async fn list_dict_types(
    Query(page_params): Query<PageParamsOption>,
    Query(search_params): Query<DictTypeSearchParams>,
) -> impl IntoResponse {
    info!("list_dict_types");
    let txn = db!().begin().await.unwrap();
    let result = DictTypeService::page::<DictTypePageVo>(&txn, page_params, search_params).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "查询成功").json()
}

pub async fn create_dict_type(Json(dto): Json<MergeDictTypeModel>) -> impl IntoResponse {
    info!("create_dict_type: {:?}", dto);
    let txn = db!().begin().await.unwrap();
    let result = DictTypeService::add(&txn, dto).await;
    txn.commit().await.unwrap();

    RespVO::from_result(result, "新增成功").json()
}

pub async fn get_dict_type(Path(dict_type_id): Path<i32>) -> impl IntoResponse {
    info!("get_dict_type: dict_type_id={}", dict_type_id);
    let txn = db!().begin().await.unwrap();
    let result = DictTypeService::get_by_id::<DictTypePageVo>(&txn, dict_type_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "查询成功").json()
}

pub async fn update_dict_type(
    Path(user_id): Path<i32>,
    Json(mut dto): Json<MergeDictTypeModel>,
) -> impl IntoResponse {
    dto.set_id(user_id);

    info!("update_dict_type: {:?}", dto);
    let txn = db!().begin().await.unwrap();
    let result = DictTypeService::update(&txn, dto).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "更新成功").json()
}

pub async fn delete_dict_type(Path(dict_type_id): Path<i32>) -> impl IntoResponse {
    info!("delete_dict_type: dict_type_id={}", dict_type_id);
    let txn = db!().begin().await.unwrap();
    let result = DictTypeService::delete(&txn, dict_type_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "删除成功").json()
}

pub async fn toggle_dict_type_enable(Path(dict_type_id): Path<i32>) -> impl IntoResponse {
    info!("toggle_dict_type_enable: dict_type_id={}", dict_type_id);
    let txn = db!().begin().await.unwrap();
    let result = DictTypeService::toggle_enable(&txn, dict_type_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "操作成功").json()
}
