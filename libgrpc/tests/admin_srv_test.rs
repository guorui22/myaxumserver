use tonic::Request;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use libgrpc::{generate_random_string, get_admin_client};

use libproto::{
    admin_service_client::AdminServiceClient,
    EditAdminRequest,
    get_admin_request::{ByAuth, ById, Condition}, GetAdminRequest, ListAdminRequest, ToggleAdminRequest,
};

/// 服务器地址
const TEST_ADDRESS: &'static str = "http://127.0.0.1:29029";

/// JWT 认证 Token
const TEST_JWT: &'static str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjbGllbnRfaWQiOiIwNzczMyIsImNsaWVudF9uYW1lIjoi6YOt552_IiwiZXhwIjo0ODUzMTg3MzU3fQ.febzZPRQmB8br5HVitBvZar4rf1WSf80CNtE0WtnIFQ";

#[tokio::test]
async fn test_create_admin() {
    let mut client = get_admin_client!(AdminServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let resp = client
        .create_admin(tonic::Request::new(libproto::CreateAdminRequest {
            email: format!("team-{}@axum.rs", generate_random_string(10)),
            password: "axum.rs".into(),
        }))
        .await
        .unwrap();
    let reply = resp.into_inner();
    assert!(reply.id > 0);
}

#[tokio::test]
async fn test_edit_admin() {
    let mut client = get_admin_client!(AdminServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let resp = client
        .edit_admin(tonic::Request::new(EditAdminRequest {
            id: 1,
            email: "guorui22@gmail.com".into(),
            password: "axum.rs".into(),
            new_password: Some("axum.rs".into()),
        }))
        .await
        .unwrap();
    let reply = resp.into_inner();
    println!("{:?}", reply)
}
#[tokio::test]
async fn test_toggle_admin() {
    let mut client = get_admin_client!(AdminServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let resp = client
        .toggle_admin(tonic::Request::new(ToggleAdminRequest { id: 1 }))
        .await
        .unwrap();
    let reply = resp.into_inner();
    println!("{:?}", reply)
}
#[tokio::test]
async fn test_byid_get_admin() {
    let mut client = get_admin_client!(AdminServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let condition = Condition::ById(ById {
        id: 1,
        is_del: None,
    });
    let resp = client
        .get_admin(tonic::Request::new(GetAdminRequest {
            condition: Some(condition),
        }))
        .await
        .unwrap();
    let reply = resp.into_inner();
    assert!(reply.admin.is_some());
}

#[tokio::test]
async fn test_byauth_get_admin_as_login() {
    let mut client = get_admin_client!(AdminServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let condition = Condition::ByAuth(ByAuth {
        email: "guorui22@gmail.com".into(),
        password: "axum.rs".into(),
    });
    let resp = client
        .get_admin(tonic::Request::new(GetAdminRequest {
            condition: Some(condition),
        }))
        .await
        .unwrap();
    let reply = resp.into_inner();
    assert!(reply.admin.is_some());
}

#[tokio::test]
async fn test_list_admin() {

    let mut client = get_admin_client!(AdminServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let resp = client
        .list_admin(tonic::Request::new(ListAdminRequest {
            email: Some("@gmail.com".into()),
            is_del: Some(false),
        }))
        .await
        .unwrap();
    let reply = resp.into_inner();
    assert!(reply.admins.len() > 0)
}
