use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, FromQueryResult, Default)]
pub struct AdminPageVo {
    id: i64,
    login_name: String,
    phone: String,
    email: String,
    real_name: String,
    role_ids: String,
    status: i32,
}
