use axum::{
    extract::{FromRequestParts, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use tower_sessions::Session;

pub async fn auth(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    // 获取请求的 URI 路径
    let path = request.uri().path().to_string();

    // 定义需要跳过认证的路径（白名单）
    let white_list = ["/login", "/api/captcha", "/api/login", "/static"];
    if !path.starts_with("/static") && !white_list.contains(&path.as_str()) {
        // 从请求中提取会话
        let mut parts = request.into_parts();
        let session = Session::from_request_parts(&mut parts.0, &())
            .await
            .unwrap();

        // 检查用户是否已登录
        let user_id = session.get::<i32>("user_id").await.unwrap_or(None);

        if user_id.is_none() {
            // 用户未登录，重定向到登录页面
            let response = Redirect::to("/login").into_response();
            return Ok(response);
        }

        // 将请求重新组合
        request = Request::from_parts(parts.0, parts.1);
    }

    // 继续处理请求
    let response = next.run(request).await;
    Ok(response)
}
