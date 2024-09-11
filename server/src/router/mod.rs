mod user;

use handlebars::Handlebars;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::info;
use walkdir::{DirEntry, WalkDir};

use axum::{Extension, Router};
use once_cell::sync::Lazy;

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
    Router::new().nest("/users", user::user_api_router())
}

fn init_template_router() -> Router {
    Router::new().nest("/users", user::user_template_router())
}

pub fn init_router() -> Router {
    Router::new()
        // init the template router
        .nest("/", init_template_router())
        .nest_service("/static/", ServeDir::new("./static")) // 静态文件服务
        .nest("/api", init_api_router())
        .layer(Extension(HANDLEBARS.clone()))
}
