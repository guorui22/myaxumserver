use askama::Template;
use axum::{Extension, Form};
use axum::extract::Query;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Html;
use axum_macros::debug_handler;
use deadpool_redis::redis::cmd;
use serde_json::json;
use uuid::Uuid;

use crate::database::{Redis01, RedisPool};
use crate::session::{get_session_from_cookie, LoginMessage, save_session_id_to_cookie, SESSION_KEY_PREFIX, UserLoginForm, UserSession};
use crate::template::{LoginTemplate, MainTemplate};

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
    let url =
        if !(&frm.username == "07733" && &frm.password == "123581321") {
            "/login?msg=用户名或密码错误"
        } else {
            // 生成 session ID
            let session_id = Uuid::new_v4().simple().to_string();
            // 将 session ID 保存到 Cookie
            save_session_id_to_cookie(&session_id, &mut headers);

            let user_session = UserSession {
                username: frm.username,
                level: 1,
            };
            let user_session = json!(user_session).to_string();

            // 将 session 保存到 redis
            let redis_key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
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
            "/main"
        };
    headers.insert(axum::http::header::LOCATION, url.parse().unwrap());
    Ok((StatusCode::FOUND, headers, ()))
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
        let redis_key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
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
    Extension(_pool): Extension<RedisPool<Redis01>>,
    headers: HeaderMap,
) -> Result<Html<String>, String> {
    let session_id = get_session_from_cookie(&headers).ok_or("从 Cookie 中获取 Session ID 失败。")?;
    let redis_key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
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
    let html = MainTemplate { username, level }.render().map_err(|err| {
        format!("main 模板渲染失败：{}", err)
    })?;
    Ok(Html(html))
}
