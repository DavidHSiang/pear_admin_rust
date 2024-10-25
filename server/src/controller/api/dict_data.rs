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
        vo::{DictDataPageVo, RespVO},
    },
    db,
    service::dict_data::{DictDataSearchParams, DictDataService, MergeDictDataModel},
};

pub async fn list_dict_data(
    Query(page_params): Query<PageParamsOption>,
    Query(search_params): Query<DictDataSearchParams>,
) -> impl IntoResponse {
    info!("list_dict_data");
    let txn = db!().begin().await.unwrap();
    let result = DictDataService::page::<DictDataPageVo>(&txn, page_params, search_params).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "查询成功").json()
}

pub async fn create_dict_data(Json(dto): Json<MergeDictDataModel>) -> impl IntoResponse {
    info!("create_dict_data: {:?}", dto);
    let txn = db!().begin().await.unwrap();
    let result = DictDataService::add(&txn, dto).await;
    txn.commit().await.unwrap();

    RespVO::from_result(result, "新增成功").json()
}

pub async fn get_dict_data(Path(dict_data_id): Path<i32>) -> impl IntoResponse {
    info!("get_dict_data: dict_data_id={}", dict_data_id);
    let txn = db!().begin().await.unwrap();
    let result = DictDataService::get_by_id::<DictDataPageVo>(&txn, dict_data_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "查询成功").json()
}

pub async fn update_dict_data(
    Path(user_id): Path<i32>,
    Json(mut dto): Json<MergeDictDataModel>,
) -> impl IntoResponse {
    dto.set_id(user_id);

    info!("update_dict_data: {:?}", dto);
    let txn = db!().begin().await.unwrap();
    let result = DictDataService::update(&txn, dto).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "更新成功").json()
}

pub async fn delete_dict_data(Path(dict_data_id): Path<i32>) -> impl IntoResponse {
    info!("delete_dict_data: dict_data_id={}", dict_data_id);
    let txn = db!().begin().await.unwrap();
    let result = DictDataService::delete(&txn, dict_data_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "删除成功").json()
}

pub async fn toggle_dict_data_enable(Path(dict_data_id): Path<i32>) -> impl IntoResponse {
    info!("toggle_dict_data_enable: dict_data_id={}", dict_data_id);
    let txn = db!().begin().await.unwrap();
    let result = DictDataService::toggle_enable(&txn, dict_data_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "操作成功").json()
}
