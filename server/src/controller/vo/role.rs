use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, FromQueryResult, Default)]
pub struct RolePageVo {
    pub id: i32,
    name: String,
    code: String,
    enable: i32,
    remark: Option<String>,
    details: Option<String>,
    sort: i32,
    create_at: String,
    update_at: String,

    #[sea_orm(skip)]
    pub checked: bool,
}
