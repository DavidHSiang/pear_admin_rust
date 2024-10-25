use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};
use sea_orm::TransactionTrait;
use tracing::info;

use crate::{controller::vo::DeptListVo, db, router::HANDLEBARS, service::dept::DeptService};

pub async fn create_dept_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/dept/add", &()).unwrap();
    Html(rendered)
}

pub async fn list_depts_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/dept/list", &()).unwrap();
    Html(rendered)
}

pub async fn edit_dept_page(Path(id): Path<i32>) -> impl IntoResponse {
    info!("Edit dept page: dept_id={}", id);

    let txn = db!().begin().await.unwrap();
    let dept: DeptListVo = DeptService::get_by_id(&txn, id).await.unwrap();
    txn.commit().await.unwrap();

    let data = serde_json::json!({
        "dept": dept,
    });

    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/dept/edit", &data).unwrap();
    Html(rendered)
}
