use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, FromQueryResult, Default)]
pub struct DeptListVo {
    id: i32,
    parent_id: i32,
    dept_name: String,
    sort: i32,
    leader: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    enable: i32,
    remark: Option<String>,
    address: Option<String>,
    create_at: String,
    update_at: String,
}
