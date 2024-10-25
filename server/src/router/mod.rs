mod common;
mod dept;
mod dict;
mod photo;
mod power;
mod role;
mod system_log;
mod user;

use handlebars::Handlebars;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::info;
use walkdir::{DirEntry, WalkDir};

use axum::{Extension, Router};
use once_cell::sync::Lazy;
use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, SessionManagerLayer};

use crate::middleware::auth::auth;

pub static HANDLEBARS: Lazy<Arc<Handlebars<'static>>> = Lazy::new(|| {
    let mut hb = Handlebars::new();

    load_templates(&mut hb, "./template"); // TODO: 从配置文件中读取模板路径

    Arc::new(hb)
});

fn load_templates(hb: &mut Handlebars, base_path: &str) {
    for entry in WalkDir::new(base_path).into_iter().filter_map(Result::ok) {
        let entry_path = entry.path();
        if is_template_file(&entry) {
            // 生成基于完整路径的模板名
            let template_name = entry_path
                .strip_prefix(base_path)
                .unwrap()
                .to_str()
                .unwrap()
                .replace("\\", "/") // 确保跨平台路径一致性
                .trim_end_matches(".hbs")
                .to_owned();
            info!("Loading template: {}", template_name);
            // 注册模板文件
            if let Err(e) = hb.register_template_file(&template_name, entry_path) {
                panic!("Error registering template {}: {}", template_name, e);
            }
        }
    }
}

fn is_template_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file() && entry.path().extension().map_or(false, |e| e == "hbs")
}

fn init_api_router() -> Router {
    Router::new()
        .nest("/", common::common_api_router())
        .nest("/users", user::user_api_router())
        .nest("/roles", role::role_api_router())
        .nest("/powers", power::power_api_router())
        .nest("/depts", dept::dept_api_router())
        .nest("/dicts", dict::dict_api_router())
        .nest("/system_logs", system_log::system_log_api_router())
        .nest("/photos", photo::photo_api_router())
}

fn init_template_router() -> Router {
    Router::new()
        .nest("/", common::common_template_router())
        .nest("/users", user::user_template_router())
        .nest("/roles", role::role_template_router())
        .nest("/powers", power::power_template_router())
        .nest("/depts", dept::dept_template_router())
        .nest("/dicts", dict::dict_template_router())
        .nest("/system_logs", system_log::system_log_template_router())
        .nest("/photos", photo::photo_template_router())
}

pub fn init_router() -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    Router::new()
        // init the template router
        .nest("/", init_template_router())
        .nest_service("/static/", ServeDir::new("./static")) // 静态文件服务
        .nest("/api", init_api_router())
        .layer(Extension(HANDLEBARS.clone()))
        .layer(axum::middleware::from_fn(auth))
        .layer(session_layer)
}
