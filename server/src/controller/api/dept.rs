use axum::{extract::Path, response::IntoResponse, Json};
use sea_orm::TransactionTrait;
use tracing::info;

use crate::{
    controller::vo::{DeptListVo, RespVO},
    db,
    service::dept::{DeptService, MergeDeptModel},
};

pub async fn list_depts() -> impl IntoResponse {
    info!("list_powers");
    let txn = db!().begin().await.unwrap();
    let result = DeptService::list_all::<DeptListVo>(&txn).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "查询成功").json()
}

pub async fn create_dept(Json(dto): Json<MergeDeptModel>) -> impl IntoResponse {
    info!("create_dept: {:?}", dto);
    let txn = db!().begin().await.unwrap();
    let result = DeptService::add(&txn, dto).await;
    txn.commit().await.unwrap();

    RespVO::from_result(result, "新增成功").json()
}

pub async fn get_dept(Path(dept_id): Path<i32>) -> impl IntoResponse {
    info!("get_dept: dept_id={}", dept_id);
    let txn = db!().begin().await.unwrap();
    let result = DeptService::get_by_id::<DeptListVo>(&txn, dept_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "查询成功").json()
}

pub async fn update_dept(
    Path(user_id): Path<i32>,
    Json(mut dto): Json<MergeDeptModel>,
) -> impl IntoResponse {
    dto.set_id(user_id);

    info!("update_dept: {:?}", dto);
    let txn = db!().begin().await.unwrap();
    let result = DeptService::update(&txn, dto).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "更新成功").json()
}

pub async fn delete_dept(Path(dept_id): Path<i32>) -> impl IntoResponse {
    info!("delete_dept: dept_id={}", dept_id);
    let txn = db!().begin().await.unwrap();
    let result = DeptService::delete(&txn, dept_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "删除成功").json()
}

pub async fn toggle_dept_enable(Path(dept_id): Path<i32>) -> impl IntoResponse {
    info!("toggle_dept_enable: dept_id={}", dept_id);
    let txn = db!().begin().await.unwrap();
    let result = DeptService::toggle_enable(&txn, dept_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "操作成功").json()
}
