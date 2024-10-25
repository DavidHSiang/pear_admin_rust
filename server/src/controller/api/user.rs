use crate::{
    controller::{
        dto::PageParamsOption,
        vo::{RespVO, UserPageVo},
    },
    db,
    service::user::{MergeUserModel, UserSearchParams, UserService},
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use sea_orm::TransactionTrait;
use tracing::info;

pub async fn list_users(
    Query(page_params): Query<PageParamsOption>,
    Query(search_params): Query<UserSearchParams>,
) -> impl IntoResponse {
    info!("list_users");

    let txn = db!().begin().await.unwrap();
    let result = UserService::page_with_dept::<UserPageVo>(&txn, page_params, search_params).await;
    txn.commit().await.unwrap();

    RespVO::from_result(result, "查询成功").json()
}

pub async fn create_user(Json(dto): Json<MergeUserModel>) -> impl IntoResponse {
    info!("create_user: {:?}", dto);
    let txn = db!().begin().await.unwrap();
    let result = UserService::add(&txn, dto).await;
    txn.commit().await.unwrap();

    RespVO::from_result(result, "新增成功").json()
}

pub async fn get_user(Path(user_id): Path<i32>) -> impl IntoResponse {
    info!("get_user: user_id={}", user_id);
    let txn = db!().begin().await.unwrap();
    let result = UserService::get_by_id_with_dept::<UserPageVo>(&txn, user_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "查询成功").json()
}

pub async fn update_user(
    Path(user_id): Path<i32>,
    Json(mut dto): Json<MergeUserModel>,
) -> impl IntoResponse {
    dto.set_id(user_id);

    info!("update_user: {:?}", dto);
    let txn = db!().begin().await.unwrap();
    let result = UserService::update(&txn, dto).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "更新成功").json()
}

pub async fn delete_user(Path(user_ids): Path<String>) -> impl IntoResponse {
    info!("delete_users: {}", user_ids);
    let ids: Vec<i32> = user_ids
        .split(',')
        .filter_map(|id| id.parse::<i32>().ok())
        .collect();

    let txn = db!().begin().await.unwrap();

    for user_id in &ids {
        let result = UserService::delete(&txn, *user_id).await;
        if result.is_err() {
            txn.rollback().await.unwrap();
            return RespVO::from_result(result, "删除失败").json();
        }
    }
    txn.commit().await.unwrap();
    RespVO::from((), "删除成功").json()
}

pub async fn toggle_user_enable(Path(user_id): Path<i32>) -> impl IntoResponse {
    info!("toggle_user_enable: user_id={}", user_id);
    let txn = db!().begin().await.unwrap();
    let result = UserService::toggle_enable(&txn, user_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "操作成功").json()
}
