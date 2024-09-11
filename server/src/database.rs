use sea_orm::{entity::prelude::DatabaseConnection, ConnectOptions, Database};
use std::time::Duration;
use tokio::sync::OnceCell;

//  异步初始化数据库
pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn db_conn() -> DatabaseConnection {
    let mut opt = ConnectOptions::new("sqlite://pear-admin.db");
    opt.max_connections(1000)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(false);
    let db = Database::connect(opt).await.expect("数据库打开失败");
    tracing::info!("Database connected");
    db
}

#[macro_export]
macro_rules! db {
    () => {
        $crate::database::DB
            .get_or_init($crate::database::db_conn)
            .await
    };
}

// #[cfg(test)]
// mod tests {
//     use std::env;

//     #[tokio::test]
//     async fn test_db() {
//         let root_path = env::current_dir().unwrap().join("../");
//         assert!(env::set_current_dir(&root_path).is_ok());
//         let db = db!();
//         // test connection
//         let ret = db.ping().await;
//         assert!(ret.is_ok());
//     }
// }
