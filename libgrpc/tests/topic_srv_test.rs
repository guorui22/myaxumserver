use libproto::{
    CreateTopicRequest, DatelineRange, EditTopicRequest, GetTopicRequest,
    ListTopicRequest, ToggleTopicRequest, topic_service_client::TopicServiceClient,
};
use libgrpc::get_admin_client;
use tonic::Request;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;

/// 服务器地址
const TEST_ADDRESS: &'static str = "http://127.0.0.1:29029";

/// JWT 认证 Token
const TEST_JWT: &'static str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjbGllbnRfaWQiOiIwNzczMyIsImNsaWVudF9uYW1lIjoi6YOt552_IiwiZXhwIjo0ODUzMTg3MzU3fQ.febzZPRQmB8br5HVitBvZar4rf1WSf80CNtE0WtnIFQ";

#[tokio::test]
async fn test_create_topic() {
    let mut client = get_admin_client!(TopicServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(CreateTopicRequest {
        title: "Hello,世界".into(),
        category_id: 1.into(),
        content: "欢迎来到axum.rs".into(),
        summary: None,
    });
    let resp = client.create_topic(request).await.unwrap();
    let reply = resp.into_inner();
    assert!(reply.id > 0);
}

#[tokio::test]
async fn test_edit_topic() {
    let mut client = get_admin_client!(TopicServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(EditTopicRequest {
        id: 4.into(),
        title: "Hello,世界!".into(),
        category_id: 1.into(),
        content: "欢迎来到axum.rs!".into(),
        summary: Some("axum.rs yyds".to_string()),
    });
    let resp = client.edit_topic(request).await.unwrap();
    let reply = resp.into_inner();
    assert!(reply.id == 4);
}

#[tokio::test]
async fn test_toggle_topic() {
    let mut client = get_admin_client!(TopicServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(ToggleTopicRequest { id: 1.into() });
    let resp = client.toggle_topic(request).await.unwrap();
    let reply = resp.into_inner();
    assert!(reply.id == 1);
}

#[tokio::test]
async fn test_get_topic() {
    let mut client = get_admin_client!(TopicServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(GetTopicRequest {
        id: 1.into(),
        is_del: None,
        inc_hit: None,
    });
    let resp = client.get_topic(request).await.unwrap();
    let reply = resp.into_inner();
    assert!(reply.topic.is_some());
    println!("{:?}", reply.topic.unwrap().dateline);
}
#[tokio::test]
async fn test_get_notexists_topic() {
    let mut client = get_admin_client!(TopicServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(GetTopicRequest {
        id: 1111.into(),
        is_del: None,
        inc_hit: None,
    });
    let resp = client.get_topic(request).await;
    assert!(resp.is_err());
}

#[tokio::test]
async fn test_get_topic_incr_hit() {
    let mut client = get_admin_client!(TopicServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(GetTopicRequest {
        id: 2.into(),
        is_del: Some(false),
        inc_hit: Some(true),
    });
    let resp = client.get_topic(request).await.unwrap();
    let reply = resp.into_inner();
    assert!(reply.topic.is_some());
    assert!(reply.topic.unwrap().hit > 0);
}

#[tokio::test]
async fn test_list_topic() {
    let mut client = get_admin_client!(TopicServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let request = tonic::Request::new(ListTopicRequest {
        page: None,
        category_id: None,
        keyword: None,
        is_del: None,
        dateline_range: None,
    });
    let resp = client.list_topic(request).await.unwrap();
    let reply = resp.into_inner();
    println!("RT: {}, PT: {}", reply.record_total, reply.page_totoal);
    for t in reply.topics {
        println!("{:?}", t);
    }
}

#[tokio::test]
async fn test_args_list_topic() {
    let mut client = get_admin_client!(TopicServiceClient<Channel>, TEST_ADDRESS, TEST_JWT);
    let start = prost_types::Timestamp::date_time(2022, 9, 20, 0, 0, 0).unwrap();
    let end = prost_types::Timestamp::date_time(2022, 9, 30, 23, 59, 59).unwrap();
    let request = tonic::Request::new(ListTopicRequest {
        page: Some(0),
        category_id: Some(1),
        keyword: Some("hello".into()),
        is_del: None,
        dateline_range: Some(DatelineRange {
            start: Some(start),
            end: Some(end),
        }),
    });
    let resp = client.list_topic(request).await.unwrap();
    let reply = resp.into_inner();
    println!("RT: {}, PT: {}", reply.record_total, reply.page_totoal);
    for t in reply.topics {
        println!("{:?}", t);
    }
}
