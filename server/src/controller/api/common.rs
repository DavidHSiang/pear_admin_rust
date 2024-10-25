use crate::{
    controller::vo::MenuVo,
    db,
    service::{
        common::{CommonService, LoginParams},
        monitor::MonitorService,
    },
};
use axum::{response::IntoResponse, Form, Json};
use sea_orm::TransactionTrait;
use tower_sessions::Session;
use tracing::info;

use crate::controller::vo::RespVO;

// rewrite configs() to rust
pub async fn configs() -> impl IntoResponse {
    // 网站配置
    let config = serde_json::json!({
        "logo": {
            // 网站名称 // TODO 从配置文件读取
            "title": "Pear Admin Rust",
            // 网站图标
            "image": "/static/admin/images/logo.png"
        },
        "menu": {
            // 菜单数据来源
            "data": "/api/menu",
            "collaspe": false,
            // 是否同时只打开一个菜单目录
            "accordion": true,
            "method": "GET",
            // 是否开启多系统菜单模式
            "control": false,
            // 顶部菜单宽度 PX
            "controlWidth": 500,
            // 默认选中的菜单项
            "select": "0",
            // 是否开启异步菜单，false 时 data 属性设置为菜单数据，false 时为 json 文件或后端接口
            "async": true
        },
        "tab": {
            // 是否开启多选项卡
            "muiltTab": true,
            // 切换选项卡时，是否刷新页面状态
            "keepState": true,
            // 是否开启 Tab 记忆
            "session": true,
            // 最大可打开的选项卡数量
            "tabMax": 30,
            "index": {
                // 标识 ID , 建议与菜单项中的 ID 一致
                "id": "10",
                // 页面地址
                "href": "/welcome",
                // 标题
                "title": "首页"
            }
        },
        "theme": {
            // 默认主题色，对应 colors 配置中的 ID 标识
            "defaultColor": "2",
            // 默认的菜单主题 dark-theme 黑 / light-theme 白
            "defaultMenu": "dark-theme",
            // 是否允许用户切换主题，false 时关闭自定义主题面板
            "allowCustom": true
        },
        "colors": [
            {
                "id": "1",
                "color": "#2d8cf0"
            },
            {
                "id": "2",
                "color": "#5FB878"
            },
            {
                "id": "3",
                "color": "#1E9FFF"
            },
            {
                "id": "4",
                "color": "#FFB800"
            },
            {
                "id": "5",
                "color": "darkgray"
            }
        ],
        "links": [
            {
                "href": "http://www.pearadmin.com",
                "icon": "layui-icon layui-icon-auz",
                "title": "官方网站"
            },
            {
                "href": "http://www.pearadmin.com",
                "icon": "layui-icon layui-icon-auz",
                "title": "开发文档"
            },
            {
                "href": "https://gitee.com/Jmysy/Pear-Admin-Layui",
                "icon": "layui-icon layui-icon-auz",
                "title": "Pear-Admin-Layui"
            }
        ],
        "other": {
            // 主页动画时长
            "keepLoad": 0,
            // 布局顶部主题
            "autoHead": false
        },
        "header": false
    });
    Json(config)
}

pub async fn menu(session: Session) -> impl IntoResponse {
    let txn = db!().begin().await.unwrap();
    let user_id = session.get::<i32>("user_id").await.unwrap().unwrap();
    let menu = CommonService::menu(&txn, user_id).await.unwrap();
    txn.commit().await.unwrap();

    let menu = MenuVo::to_tree(menu);
    Json(serde_json::json!(menu))
}

// TODO 目前版本存在漏洞，uuid 并没有过期时间，后续版本会修复
pub async fn captcha() -> impl IntoResponse {
    info!("captcha");
    let result = CommonService::captcha().await;
    RespVO::from_result(result, "获取验证码成功").json()
}

pub async fn login(session: Session, Form(login_params): Form<LoginParams>) -> impl IntoResponse {
    info!("login");

    let txn = db!().begin().await.unwrap();
    let result = CommonService::login(&txn, login_params).await;
    txn.commit().await.unwrap();

    match result {
        Ok(user_id) => {
            // insert user_id into session
            session.clear().await;
            session.insert("user_id", user_id).await.unwrap();
            RespVO::from((), "登录成功")
        }
        Err(e) => RespVO::from_error(e.to_string()),
    }
    .json()
}

pub async fn monitor_polling() -> impl IntoResponse {
    let sysinfo = MonitorService::monitor_polling().await;
    Json(sysinfo)
}
