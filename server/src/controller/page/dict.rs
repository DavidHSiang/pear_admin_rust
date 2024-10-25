use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};
use sea_orm::TransactionTrait;
use tracing::info;

use crate::{
    controller::vo::{DictDataPageVo, DictTypePageVo},
    db,
    router::HANDLEBARS,
    service::{dict_data::DictDataService, dict_type::DictTypeService},
};

pub async fn list_dict_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/dict/list", &()).unwrap();
    Html(rendered)
}

pub async fn create_dict_type_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/dict/dict_type/add", &()).unwrap();
    Html(rendered)
}

pub async fn edit_dict_type_page(Path(id): Path<i32>) -> impl IntoResponse {
    info!("Edit dict type page: dict_type_id={}", id);
    let txn = db!().begin().await.unwrap();
    let dict_type: DictTypePageVo = DictTypeService::get_by_id(&txn, id).await.unwrap();
    txn.commit().await.unwrap();

    let data = serde_json::json!({ "dict_type": dict_type });

    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars
        .render("system/dict/dict_type/edit", &data)
        .unwrap();
    Html(rendered)
}

pub async fn create_dict_data_page(Path(type_code): Path<String>) -> impl IntoResponse {
    let context = serde_json::json!({ "type_code": type_code });

    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars
        .render("system/dict/dict_data/add", &context)
        .unwrap();
    Html(rendered)
}

pub async fn edit_dict_data_page(Path(id): Path<i32>) -> impl IntoResponse {
    info!("Edit dict data page: dict_data_id={}", id);
    let txn = db!().begin().await.unwrap();
    let dict_data: DictDataPageVo = DictDataService::get_by_id(&txn, id).await.unwrap();
    txn.commit().await.unwrap();

    let data = serde_json::json!({ "dict_data": dict_data });

    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars
        .render("system/dict/dict_data/edit", &data)
        .unwrap();
    Html(rendered)
}
