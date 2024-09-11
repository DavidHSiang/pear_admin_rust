use crate::{
    controller::{
        dto::{AdminAddDTO, AdminPageDTO},
        vo::{AdminPageVo, RespVO},
    },
    db,
    service::user::AdminService,
};
use axum::{extract::Query, response::IntoResponse, Json};
use sea_orm::TransactionTrait;
use tracing::info;

pub async fn list_users(Query(dto): Query<AdminPageDTO>) -> impl IntoResponse {
    info!("list_users: {:?}", dto);

    let txn = db!().begin().await.unwrap();
    let result = AdminService::page::<AdminPageVo>(&txn, dto).await;
    txn.commit().await.unwrap();

    RespVO::from_result(result, "查询成功").json()
}

pub async fn create_user(Json(dto): Json<AdminAddDTO>) -> impl IntoResponse {
    info!("create_user: {:?}", dto);

    let txn = db!().begin().await.unwrap();
    let result = AdminService::add(&txn, dto).await;
    txn.commit().await.unwrap();

    RespVO::from_result(result, "新增成功").json()
}

pub async fn get_user() -> impl IntoResponse {
    "Get a specific user"
}

pub async fn update_user() -> impl IntoResponse {
    "Update a user"
}

pub async fn delete_user() -> impl IntoResponse {
    "Delete a user"
}
