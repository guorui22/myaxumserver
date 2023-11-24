use askama::Template;
use axum::{Extension, Form};
use axum::extract::Query;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Html;
use axum_macros::debug_handler;
use deadpool_redis::redis::cmd;
use serde_json::json;

use crate::auth::{Claims, get_auth_user, get_session_from_cookie, JWT, save_session_id_to_cookie, SESSION_PREFIX_FOR_REDIS};
use crate::database::{Redis01, RedisPool};
use crate::handler::{LoginMessage, LoginTemplate, MainTemplate, UserLoginForm, UserSession};

/// Session场景-登录界面
#[debug_handler]
pub async fn user_login(Query(login_msg): Query<LoginMessage>) -> Result<Html<String>, String> {
    let msg = match login_msg.msg {
        None => "".to_string(),
        Some(msg) => msg,
    };
    let login_template = LoginTemplate { msg };
    let html = login_template.render().map_err(|err| {
        format!("login 模板渲染失败：{}", err)
    })?;
    Ok(Html(html))
}

/// Session场景-登录操作
pub async fn login_action(
    Extension(pool): Extension<RedisPool<Redis01>>,
    Form(frm): Form<UserLoginForm>,
) -> Result<(StatusCode, HeaderMap, ()), String> {
    let mut headers = HeaderMap::new();

    return if let Some(user) = get_auth_user(&frm.usercode, &frm.password) {
        // 获取用户名
        let client_name = user.get("name").map_or("", |val| val.as_str().unwrap_or_default());
        // 构造 token 重要包含的信息(token 过期时间很重要)
        let claims = JWT.new_claims(frm.usercode.clone(), client_name.to_string(), 30);
        if let Ok(token) = JWT.token(&claims) {
            // 将 session ID 保存到 Cookie
            save_session_id_to_cookie(&token, &mut headers);

            let user_session = UserSession {
                username: client_name.to_string(),
                level: 1,
            };
            let user_session = json!(user_session).to_string();

            // 将 session 保存到 redis
            let redis_key = format!("{}{}", SESSION_PREFIX_FOR_REDIS, token);
            let mut conn = pool.get().await.map_err(|err| {
                format!("Redis 获取连接失败：{}", err)
            })?;
            // session 将在20分钟后自动过期
            cmd("SETEX")
                .arg(redis_key.clone())
                .arg(1200)
                .arg(user_session)
                .query_async::<_, ()>(&mut conn)
                .await
                .map_err(|err| format!("Redis 设置失效时间失败：{}", err))?;
            headers.insert(axum::http::header::LOCATION, "/main".parse().unwrap());
            Ok((StatusCode::FOUND, headers, ()))
        } else {
            headers.insert(axum::http::header::LOCATION, "/login?msg=生成 Token 时发生错误".parse().unwrap());
            Ok((StatusCode::FOUND, headers, ()))
        }
    } else {
        headers.insert(axum::http::header::LOCATION, "/login?msg=用户名或密码错误".parse().unwrap());
        Ok((StatusCode::FOUND, headers, ()))
    }
}

/// 退出登录
pub async fn logout_action(
    Extension(pool): Extension<RedisPool<Redis01>>,
    headers: HeaderMap,
) -> Result<(StatusCode, HeaderMap, ()), String> {
    let session_id = get_session_from_cookie(&headers).ok_or("从 Cookie 中获取 Session ID 失败。")?;
    let mut headers = HeaderMap::new();
    {
        // 从 redis 删除 Session
        let redis_key = format!("{}{}", SESSION_PREFIX_FOR_REDIS, session_id);
        let mut conn = pool.get().await.map_err(|err| {
            format!("Redis 获取连接失败：{}", err)
        })?;
        cmd("DEL")
            .arg(redis_key)
            .query_async(&mut conn)
            .await
            .map_err(|err| format!("从 Redis 删除 Session 失败：{err}"))?;

        // 清空Cookie
        save_session_id_to_cookie("", &mut headers);
    }
    headers.insert(axum::http::header::LOCATION, "/login".parse().unwrap());
    Ok((StatusCode::FOUND, headers, ()))
}

/// Session场景-用户首页界面
#[debug_handler]
pub async fn user_main(
    _claims: Claims,
    Extension(_pool): Extension<RedisPool<Redis01>>,
    headers: HeaderMap,
) -> Result<Html<String>, String> {
    let session_id = get_session_from_cookie(&headers).ok_or("从 Cookie 中获取 Session ID 失败。")?;
    let redis_key = format!("{}{}", SESSION_PREFIX_FOR_REDIS, session_id);
    let mut conn = _pool.get().await.map_err(|err| {
        format!("Redis 获取连接失败：{}", err)
    })?;
    let session_str: String = cmd("GET")
        .arg(redis_key)
        .query_async(&mut conn)
        .await
        .map_err(|err| format!("从 Redis 获取 Session 失败：{err}"))?;
    let UserSession { username, level } =
        serde_json::from_str(&session_str).map_err(|err| format!("反序列化 UserSession 失败：{err}"))?;
    let html = MainTemplate { username, usercode: _claims.id, level }.render().map_err(|err| {
        format!("main 模板渲染失败：{}", err)
    })?;
    Ok(Html(html))
}
