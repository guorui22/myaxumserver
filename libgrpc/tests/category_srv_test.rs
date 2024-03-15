use libgrpc::{generate_random_string, get_grpc_client};
use libproto::{
    category_service_client::CategoryServiceClient, CreateCategoryRequest, EditCategoryRequest,
    GetCategoryRequest, ListCategoryRequest, ToggleCategoryRequest,
};
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::Request;

/// 服务器地址
const TEST_ADDRESS: &'static str = "http://127.0.0.1:29029";

/// JWT 认证 Token
const TEST_JWT: &'static str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjbGllbnRfaWQiOiIwNzczMyIsImNsaWVudF9uYW1lIjoi6YOt552_IiwiZXhwIjo0ODUzMTg3MzU3fQ.febzZPRQmB8br5HVitBvZar4rf1WSf80CNtE0WtnIFQ";

#[tokio::test]
async fn test_create_category() {
    let mut client = get_grpc_client!(CategoryServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(CreateCategoryRequest {
        name: format!("分类-{}", generate_random_string(5)),
    });
    let reply = client.create_category(request).await.unwrap();
    let reply = reply.into_inner();
    assert!(reply.id > 0);
}
#[tokio::test]
async fn test_edit_category() {
    let mut client = get_grpc_client!(CategoryServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(EditCategoryRequest {
        id: 1,
        name: "axum.rs".into(),
    });
    let reply = client.edit_category(request).await.unwrap();
    let reply = reply.into_inner();
    assert!(reply.id > 0);
    assert!(reply.ok);
}
#[tokio::test]
async fn test_get_category() {
    let mut client = get_grpc_client!(CategoryServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(GetCategoryRequest {
        id: 1,
        is_del: None,
    });
    let reply = client.get_category(request).await.unwrap();
    let reply = reply.into_inner();
    assert!(reply.category.is_some());
    assert_eq!(reply.category.unwrap().id, 1);
}
#[tokio::test]
async fn test_get_notexists_category() {
    let mut client = get_grpc_client!(CategoryServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(GetCategoryRequest {
        id: 100,
        is_del: Some(true),
    });
    let reply = client.get_category(request).await.unwrap();
    let reply = reply.into_inner();
    assert!(reply.category.is_none());
}

#[tokio::test]
async fn test_delete_category() {
    let mut client = get_grpc_client!(CategoryServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(GetCategoryRequest {
        id: 1,
        is_del: None,
    });
    let reply_old = client.get_category(request).await.unwrap();
    let reply_old = reply_old.into_inner();
    let request = tonic::Request::new(ToggleCategoryRequest { id: 1 });
    let reply = client.toggle_category(request).await.unwrap();
    let reply = reply.into_inner();
    assert_eq!(reply.id, 1);
    assert_eq!(reply.is_del, !reply_old.category.unwrap().is_del);
}
#[tokio::test]
async fn test_undelete_category() {
    let mut client = get_grpc_client!(CategoryServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(GetCategoryRequest {
        id: 1,
        is_del: None,
    });
    let reply_old = client.get_category(request).await.unwrap();
    let reply_old = reply_old.into_inner();
    let request = tonic::Request::new(ToggleCategoryRequest { id: 1 });
    let reply = client.toggle_category(request).await.unwrap();
    let reply = reply.into_inner();
    assert_eq!(reply.id, 1);
    assert_eq!(reply.is_del, !reply_old.category.unwrap().is_del);
}

#[tokio::test]
async fn test_delete_notexists_category() {
    let mut client = get_grpc_client!(CategoryServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(ToggleCategoryRequest { id: 100 });
    let reply = client.toggle_category(request).await;
    assert!(reply.is_err());
}

#[tokio::test]
async fn test_list_category() {
    let mut client = get_grpc_client!(CategoryServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(ListCategoryRequest {
        name: Some("小说".to_string()),
        is_del: Some(false),
    });
    let reply = client.list_category(request).await.unwrap();
    let reply = reply.into_inner();
    assert!(reply.categories.len() > 0);
}
