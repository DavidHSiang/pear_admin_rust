use axum::{extract::Query, response::IntoResponse};
use sea_orm::TransactionTrait;
use tracing::info;

use crate::{
    controller::{
        dto::PageParamsOption,
        vo::{RespVO, SystemLogPageVo},
    },
    db,
    service::system_log::{SystemLogSearchParams, SystemLogService},
};

pub async fn list_system_logs(
    Query(page_params): Query<PageParamsOption>,
    Query(search_params): Query<SystemLogSearchParams>,
) -> impl IntoResponse {
    info!("list_system_logs");
    let txn = db!().begin().await.unwrap();
    let result = SystemLogService::page::<SystemLogPageVo>(&txn, page_params, search_params).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "查询成功").json()
}
