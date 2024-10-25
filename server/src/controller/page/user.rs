use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};
use sea_orm::TransactionTrait;
use tracing::info;

use crate::{
    controller::vo::{RolePageVo, UserPageVo},
    db,
    router::HANDLEBARS,
    service::{role::RoleService, user::UserService},
};

pub async fn create_user_page() -> impl IntoResponse {
    let txn = db!().begin().await.unwrap();
    let roles: Vec<RolePageVo> = RoleService::list_all(&txn).await.unwrap();
    txn.commit().await.unwrap();

    let data = serde_json::json!({ "roles": roles });

    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/user/add", &data).unwrap();
    Html(rendered)
}

pub async fn list_users_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/user/list", &()).unwrap();
    Html(rendered)
}

pub async fn edit_user_page(Path(id): Path<i32>) -> impl IntoResponse {
    info!("Edit user page: user_id={}", id);
    let txn = db!().begin().await.unwrap();
    let user: UserPageVo = UserService::get_by_id_with_dept(&txn, id).await.unwrap();
    let mut roles: Vec<RolePageVo> = RoleService::list_all(&txn).await.unwrap();
    let checked_roles = RoleService::get_role_ids_by_user_id(&txn, id)
        .await
        .unwrap();
    txn.commit().await.unwrap();

    roles.iter_mut().for_each(|role| {
        if checked_roles.contains(&role.id) {
            role.checked = true;
        }
    });

    let data = serde_json::json!({
        "roles": roles,
        "user": user,
    });

    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/user/edit", &data).unwrap();
    Html(rendered)
}
