use std::path::Path;

use askama::Template;
use axum::{Extension, Form, Json};
use axum::extract::{Multipart, Query};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Html;
use axum_macros::debug_handler;
use deadpool_redis::redis::cmd;
use sea_orm::{ConnectionTrait, DatabaseBackend, FromQueryResult, TransactionTrait};
use serde_json::json;
#[allow(unused_imports)]
use tracing::{event, info, instrument, Level, trace};
use uuid::Uuid;

use crate::jwt::{AuthInfo, authorize, Claims};
use crate::mysql::MySQL01Pool;
use crate::redis::{get_redis_connection, Redis01Pool};
use crate::session::{get_session_from_cookie, LoginMessage, save_session_id_to_cookie, SESSION_KEY_PREFIX, UserLoginForm, UserSession};
use crate::share::{DbBatchQueryArgs, get_request_id, UploadPath};
use crate::template::{LoginTemplate, MainTemplate};
use crate::upload_file::{cn, UploadFileTemplate};

/// 测试函数
#[debug_handler]
pub async fn index() -> &'static str {
    "Welcome to chronology!"
}

/// mysql 批量查询
#[debug_handler]
pub async fn mysql_query(
    headers: HeaderMap,
    Extension(mysql_01_pool): Extension<MySQL01Pool>,
    Json(args): Json<DbBatchQueryArgs>,
) -> Result<Json<serde_json::Value>, String> {
    let request_id = get_request_id(&headers);
    info!("x-request-id={}", request_id);

    let sql_vec = args.str_sql_array;
    let conn = match args.str_node_env.as_str() {
        "prd" => mysql_01_pool.clone(),
        evn_name => Err(format!("未知的运行环境名称: {}", evn_name))?,
    };

    let mut rst_vec = vec![];
    for sql in sql_vec.into_iter() {
        let vec_colum_values = conn
            .query_all(sea_orm::Statement::from_string(DatabaseBackend::MySql, sql))
            .await
            .map_err(|err|format!("数据库查询失败: {}", err))?
            .into_iter()
            .map(|ref q1| sea_orm::query::JsonValue::from_query_result(q1, ""))
            .map(|j1| {
                if j1.is_ok() {
                    unsafe { j1.unwrap_unchecked() }
                } else {
                    sea_orm::JsonValue::String(format!("{:?}", j1))
                }
            })
            .collect::<sea_orm::JsonValue>();
        rst_vec.push(vec_colum_values);
    }

    Ok(Json(json!({
        "status": 0,
        "result": rst_vec,
    })))
}

/// mysql 批量事务
#[debug_handler]
pub async fn mysql_transaction(
    headers: HeaderMap,
    Extension(mysql_01_pool): Extension<MySQL01Pool>,
    Json(args): Json<DbBatchQueryArgs>,
) -> Result<Json<serde_json::Value>, String> {
    let request_id = get_request_id(&headers);
    info!("x-request-id={}", request_id);

    let sql_vec = args.str_sql_array;
    let conn = match args.str_node_env.as_str() {
        "prd" => mysql_01_pool,
        evn_name => Err(format!("未知的运行环境名称: {}", evn_name))?,
    };

    // 开启事务
    let db_transaction = conn.begin().await.map_err(|err|format!("开启数据库事务失败: {}", err))?;

    let mut rst_vec = vec![];
    for sql in sql_vec.into_iter() {
        let exec_rst = db_transaction
            .execute(sea_orm::Statement::from_string(DatabaseBackend::MySql, sql))
            .await
            .map_err(|err|format!("数据库SQL执行失败: {}", err))?;
        rst_vec.push(json!({ "rows_affected":exec_rst.rows_affected(), "last_insert_id": exec_rst.last_insert_id()}));
    }

    // 结束事务
    db_transaction
        .commit()
        .await
        .map_err(|err|format!("数据库事务提交失败: {}", err))?;

    Ok(Json(json!({
        "status": 0,
        "result": rst_vec,
    })))
}

/// 使用用户名&密码获取 JWT Token
#[debug_handler]
pub async fn get_jwt_token(Json(auth_info): Json<AuthInfo>) -> Result<Json<serde_json::Value>, String> {
    let auth_token = authorize(auth_info).map_err(|err|format!("用户Token验证失败: {}", err))?;
    Ok(Json(json!({
        "status": 0,
        "result": auth_token,
    })))
}

/// 使用 JWT Token 访问受保护的内容
#[debug_handler]
pub async fn get_protected_content(
    claims: Claims,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, String> {
    let request_id = get_request_id(&headers);
    info!("x-request-id={}", request_id);
    info!("claims={:?}", claims);

    Ok(Json(json!({
        "status": 0,
        "result": "Welcome to protected chronology!",
    })))
}

/// 本站页面跳转
#[debug_handler]
pub async fn redirect01() -> (StatusCode, HeaderMap, ()) {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::LOCATION,
        "/templates/redirect.html".parse().unwrap(),
    );
    (StatusCode::FOUND, headers, ())
}

/// 外站页面跳转
#[debug_handler]
pub async fn redirect02() -> (StatusCode, HeaderMap, ()) {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::LOCATION,
        "https://www.baidu.com".parse().unwrap(),
    );
    (StatusCode::FOUND, headers, ())
}

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
    Extension(pool): Extension<Redis01Pool>,
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
    Extension(Redis01Pool(pool)): Extension<Redis01Pool>,
    headers: HeaderMap,
) -> Result<(StatusCode, HeaderMap, ()), String> {
    let session_id = get_session_from_cookie(&headers).ok_or("从 Cookie 中获取 Session ID 失败。")?;
    let mut headers = HeaderMap::new();
    {
        // 从 redis 删除 Session
        let redis_key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
        let mut conn = get_redis_connection(pool).await?;
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
    Extension(Redis01Pool(_pool)): Extension<Redis01Pool>,
    headers: HeaderMap,
) -> Result<Html<String>, String> {
    let session_id = get_session_from_cookie(&headers).ok_or("从 Cookie 中获取 Session ID 失败。")?;
    let redis_key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
    let mut conn = get_redis_connection(_pool).await?;
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

/// 文件上传页面
#[debug_handler]
pub async fn upload_file() -> Result<Html<String>, String> {
    let html = UploadFileTemplate {}.render().map_err(|err| {
        format!("upload_file 模板渲染失败：{}", err)
    })?;
    Ok(Html(html))
}

/// 文件上传操作
pub async fn upload_file_action(
    Extension(UploadPath { upload_path }): Extension<UploadPath>,
    mut multipart: Multipart,
) -> Result<(HeaderMap, String), String> {
    let mut rst = vec![];
    #[allow(clippy::never_loop)]
    while let Some(file) = multipart.next_field().await.map_err(|err|err.to_string())? {
        let filename = format!("{}-{}", Uuid::new_v4().as_simple(), file.file_name().ok_or("获取文件名称出错。")?); // 上传的文件名
        let upload_path = Path::new(&upload_path).join(&filename); //
        let data = file.bytes().await.map_err(|err| {
            format!("获取上传文件内容失败：{}", err)
        })?; // 上传的文件的内容

        if data.is_empty() {
            continue;
        }

        // 保存上传的文件
        tokio::fs::write(upload_path.clone(), &data)
            .await
            .map_err(|err| {
                format!("保存上传文件到磁盘失败：{}", err)
            })?;

        rst.push(format!(
            "【上传的文件】文件名：{:?}, 文件大小：{}",
            filename,
            data.len()
        ));
    }
    if rst.is_empty(){
        rst.push(String::from("没有上传文件"))
    }

    cn(json!(rst).to_string()).await

}