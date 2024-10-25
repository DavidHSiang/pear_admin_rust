use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, FromQueryResult, Default)]
pub struct PowerListVo {
    id: i32,
    name: String,
    r#type: String,
    code: Option<String>,
    url: Option<String>,
    open_type: Option<String>,
    parent_id: i32,
    icon: Option<String>,
    sort: i32,
    create_at: String,
    update_at: String,
    enable: i32,
}

#[derive(Serialize, Debug, Clone, FromQueryResult, Default)]
pub struct PowerListWithTopLevelVo {
    pub id: i32,
    pub name: String,
    pub parent_id: i32,
}
