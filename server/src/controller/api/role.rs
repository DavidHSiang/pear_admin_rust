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
        vo::{MenuVo, RespVO, RolePageVo},
    },
    db,
    service::{
        power::PowerService,
        role::{MergeRoleModel, RoleSearchParams, RoleService},
        role_power::RolePowerService,
    },
};

pub async fn list_roles(
    Query(page_params): Query<PageParamsOption>,
    Query(search_params): Query<RoleSearchParams>,
) -> impl IntoResponse {
    info!("list_roles: {:?}", search_params);

    let txn = db!().begin().await.unwrap();
    let result = RoleService::page::<RolePageVo>(&txn, page_params, search_params).await;
    txn.commit().await.unwrap();

    RespVO::from_result(result, "查询成功").json()
}

pub async fn create_role(Json(dto): Json<MergeRoleModel>) -> impl IntoResponse {
    info!("create_role: {:?}", dto);
    let txn = db!().begin().await.unwrap();
    let result = RoleService::add(&txn, dto).await;
    txn.commit().await.unwrap();

    RespVO::from_result(result, "新增成功").json()
}

pub async fn get_role(Path(role_id): Path<i32>) -> impl IntoResponse {
    info!("get_role: role_id={}", role_id);
    let txn = db!().begin().await.unwrap();
    let result = RoleService::get_by_id::<RolePageVo>(&txn, role_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "查询成功").json()
}

pub async fn update_role(
    Path(user_id): Path<i32>,
    Json(mut dto): Json<MergeRoleModel>,
) -> impl IntoResponse {
    dto.set_id(user_id);

    info!("update_role: {:?}", dto);
    let txn = db!().begin().await.unwrap();
    let result = RoleService::update(&txn, dto).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "更新成功").json()
}

pub async fn delete_role(Path(role_id): Path<i32>) -> impl IntoResponse {
    info!("delete_role: {}", role_id);
    let txn = db!().begin().await.unwrap();
    let result = RoleService::delete(&txn, role_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "删除成功").json()
}

pub async fn list_role_powers(Path(role_id): Path<i32>) -> impl IntoResponse {
    info!("list_role_powers: role_id={}", role_id);
    let txn = db!().begin().await.unwrap();
    let power_ids = RolePowerService::get_power_ids_by_role_id(&txn, role_id)
        .await
        .unwrap();

    let mut all_powers: Vec<MenuVo> = PowerService::list_all(&txn).await.unwrap();

    for power in all_powers.iter_mut() {
        if power_ids.contains(&power.id) {
            power.checked = "1".to_string();
        } else {
            power.checked = "0".to_string();
        }
    }

    txn.commit().await.unwrap();
    RespVO::from(all_powers, "查询成功").json()
}

pub async fn update_powers_for_role(
    Path(role_id): Path<i32>,
    Json(power_ids): Json<Vec<i32>>,
) -> impl IntoResponse {
    info!(
        "update_powers_for_role: role_id={}, power_ids={:?}",
        role_id, power_ids
    );
    let txn = db!().begin().await.unwrap();
    let result = RolePowerService::update_powers_for_role(&txn, role_id, power_ids).await;

    txn.commit().await.unwrap();
    RespVO::from_result(result, "设置成功").json()
}

pub async fn toggle_role_enable(Path(role_id): Path<i32>) -> impl IntoResponse {
    info!("toggle_role_enable: role_id={}", role_id);
    let txn = db!().begin().await.unwrap();
    let result = RoleService::toggle_enable(&txn, role_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "设置成功").json()
}
