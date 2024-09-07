use anyhow::Result;

use pear_admin_rust::init_router;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let art = r#"
Welcome to pear-admin-rust, a simple admin panel written in Rust.
______                  ___      _           _        ______          _
| ___ \                / _ \    | |         (_)       | ___ \        | |
| |_/ /__  __ _ _ __  / /_\ \ __| |_ __ ___  _ _ __   | |_/ /   _ ___| |_
|  __/ _ \/ _` | '__| |  _  |/ _` | '_ ` _ \| | '_ \  |    / | | / __| __|
| | |  __/ (_| | |    | | | | (_| | | | | | | | | | | | |\ \ |_| \__ \ |_
\_|  \___|\__,_|_|    \_| |_/\__,_|_| |_| |_|_|_| |_| \_| \_\__,_|___/\__|"#;
    println!("{}", art);

    let port = 3000; // TODO: 从配置文件中读取端口
    let addr = format!("0.0.0.0:{}", port); // TODO: 从配置文件中设置是否允许外部访问
    info!("Server is listening on http://127.0.0.1:{}", port);
    // create a new axum app
    let app = init_router();
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
