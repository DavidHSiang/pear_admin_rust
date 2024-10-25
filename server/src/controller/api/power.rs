use axum::{extract::Path, response::IntoResponse, Json};
use sea_orm::TransactionTrait;
use tracing::info;

use crate::{
    controller::vo::{PowerListVo, PowerListWithTopLevelVo, RespVO},
    db,
    service::power::{MergePowerModel, PowerService},
};

pub async fn list_powers() -> impl IntoResponse {
    info!("list_powers");
    let txn = db!().begin().await.unwrap();
    let result = PowerService::list_all::<PowerListVo>(&txn).await.unwrap();
    txn.commit().await.unwrap();
    RespVO::from(result, "查询成功").json()
}

pub async fn list_powers_with_top_level() -> impl IntoResponse {
    info!("list_powers");
    let txn = db!().begin().await.unwrap();
    let mut result = PowerService::list_all::<PowerListWithTopLevelVo>(&txn)
        .await
        .unwrap();
    result.insert(
        0,
        PowerListWithTopLevelVo {
            id: 0,
            name: "顶级权限".to_string(),
            parent_id: -1,
        },
    );
    txn.commit().await.unwrap();
    RespVO::from(result, "查询成功").json()
}

pub async fn create_power(Json(dto): Json<MergePowerModel>) -> impl IntoResponse {
    info!("create_power: {:?}", dto);
    let txn = db!().begin().await.unwrap();
    let result = PowerService::add(&txn, dto).await;
    txn.commit().await.unwrap();

    RespVO::from_result(result, "新增成功").json()
}

pub async fn get_power(Path(power_id): Path<i32>) -> impl IntoResponse {
    info!("get_power: power_id={}", power_id);
    let txn = db!().begin().await.unwrap();
    let result = PowerService::get_by_id::<PowerListVo>(&txn, power_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "查询成功").json()
}

pub async fn update_power(
    Path(user_id): Path<i32>,
    Json(mut dto): Json<MergePowerModel>,
) -> impl IntoResponse {
    dto.set_id(user_id);

    info!("update_power: {:?}", dto);
    let txn = db!().begin().await.unwrap();
    let result = PowerService::update(&txn, dto).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "更新成功").json()
}

pub async fn delete_power(Path(power_id): Path<i32>) -> impl IntoResponse {
    info!("delete_power: {}", power_id);
    let txn = db!().begin().await.unwrap();
    let result = PowerService::delete(&txn, power_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "删除成功").json()
}

pub async fn toggle_power_enable(Path(power_id): Path<i32>) -> impl IntoResponse {
    info!("toggle_power_enable: {}", power_id);
    let txn = db!().begin().await.unwrap();
    let result = PowerService::toggle_enable(&txn, power_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "操作成功").json()
}
