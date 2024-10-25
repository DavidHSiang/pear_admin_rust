use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};
use sea_orm::TransactionTrait;
use tracing::info;

use crate::{controller::vo::PowerListVo, db, router::HANDLEBARS, service::power::PowerService};

pub async fn create_power_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/power/add", &()).unwrap();
    Html(rendered)
}

pub async fn list_powers_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/power/list", &()).unwrap();
    Html(rendered)
}

pub async fn edit_power_page(Path(id): Path<i32>) -> impl IntoResponse {
    info!("Edit power page: power_id={}", id);
    let txn = db!().begin().await.unwrap();
    let power: PowerListVo = PowerService::get_by_id(&txn, id).await.unwrap();
    txn.commit().await.unwrap();

    let data = serde_json::json!({
        "power": power,
    });

    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/power/edit", &data).unwrap();
    Html(rendered)
}
