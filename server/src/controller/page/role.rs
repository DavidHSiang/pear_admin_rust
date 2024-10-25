use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};
use sea_orm::TransactionTrait;
use tracing::info;

use crate::{controller::vo::RolePageVo, db, router::HANDLEBARS, service::role::RoleService};

pub async fn create_role_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/role/add", &()).unwrap();
    Html(rendered)
}

pub async fn list_roles_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/role/list", &()).unwrap();
    Html(rendered)
}

pub async fn edit_role_page(Path(id): Path<i32>) -> impl IntoResponse {
    info!("Edit role page: role_id={}", id);

    let txn = db!().begin().await.unwrap();
    let role: RolePageVo = RoleService::get_by_id(&txn, id).await.unwrap();
    txn.commit().await.unwrap();

    let data = serde_json::json!({
        "role": role,
    });

    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/role/edit", &data).unwrap();
    Html(rendered)
}

pub async fn set_powers_for_role_page(Path(id): Path<i32>) -> impl IntoResponse {
    let data = serde_json::json!({
        "id": id,
    });

    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/role/power", &data).unwrap();
    Html(rendered)
}
