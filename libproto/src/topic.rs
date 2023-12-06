/// -- 文章
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Topic {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub category_id: i32,
    #[prost(string, tag = "4")]
    pub summary: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub content: ::prost::alloc::string::String,
    #[prost(int32, tag = "6")]
    pub hit: i32,
    #[prost(bool, tag = "7")]
    pub is_del: bool,
    #[prost(message, optional, tag = "8")]
    pub dateline: ::core::option::Option<::prost_types::Timestamp>,
}
/// -- 时间范围
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatelineRange {
    #[prost(message, optional, tag = "1")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "2")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
}
/// -- 创建文章（请求消息、响应消息）
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTopicRequest {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub category_id: i32,
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    /// 如果没有提供摘要，则自动从内容中截取
    #[prost(string, optional, tag = "4")]
    pub summary: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTopicReply {
    #[prost(int64, tag = "1")]
    pub id: i64,
}
/// -- 修改文章（请求消息、响应消息）
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditTopicRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub category_id: i32,
    /// 如果没有提供摘要，则自动从内容中截取
    #[prost(string, optional, tag = "4")]
    pub summary: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "5")]
    pub content: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditTopicReply {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(bool, tag = "2")]
    pub ok: bool,
}
/// -- 文章列表（请求消息、响应消息）
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicRequest {
    /// 页码
    #[prost(int32, optional, tag = "1")]
    pub page: ::core::option::Option<i32>,
    /// 分类
    #[prost(int32, optional, tag = "2")]
    pub category_id: ::core::option::Option<i32>,
    /// 关键字
    #[prost(string, optional, tag = "3")]
    pub keyword: ::core::option::Option<::prost::alloc::string::String>,
    /// 是否删除
    #[prost(bool, optional, tag = "4")]
    pub is_del: ::core::option::Option<bool>,
    /// 时间区间
    #[prost(message, optional, tag = "5")]
    pub dateline_range: ::core::option::Option<DatelineRange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicReply {
    /// 当前页码
    #[prost(int32, tag = "1")]
    pub page: i32,
    /// 每页条数
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// 总页数
    #[prost(int64, tag = "3")]
    pub page_totoal: i64,
    /// 总记录数
    #[prost(int64, tag = "4")]
    pub record_total: i64,
    /// 文章列表
    #[prost(message, repeated, tag = "5")]
    pub topics: ::prost::alloc::vec::Vec<Topic>,
}
/// -- 删除/恢复文章（请求消息、响应消息）
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToggleTopicRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToggleTopicReply {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(bool, tag = "2")]
    pub is_del: bool,
}
/// -- 获取文章详情（请求消息、响应消息）
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopicRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(bool, optional, tag = "2")]
    pub is_del: ::core::option::Option<bool>,
    /// 是否同时增加点击量
    #[prost(bool, optional, tag = "3")]
    pub inc_hit: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopicReply {
    #[prost(message, optional, tag = "1")]
    pub topic: ::core::option::Option<Topic>,
}
/// Generated client implementations.
pub mod topic_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// -- 在“文章”上定义rpc服务接口
    #[derive(Debug, Clone)]
    pub struct TopicServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TopicServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TopicServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TopicServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            TopicServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// 创建文章
        pub async fn create_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTopicRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTopicReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/topic.TopicService/CreateTopic",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("topic.TopicService", "CreateTopic"));
            self.inner.unary(req, path, codec).await
        }
        /// 修改文章
        pub async fn edit_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::EditTopicRequest>,
        ) -> std::result::Result<tonic::Response<super::EditTopicReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/topic.TopicService/EditTopic",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("topic.TopicService", "EditTopic"));
            self.inner.unary(req, path, codec).await
        }
        /// 文章列表
        pub async fn list_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTopicRequest>,
        ) -> std::result::Result<tonic::Response<super::ListTopicReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/topic.TopicService/ListTopic",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("topic.TopicService", "ListTopic"));
            self.inner.unary(req, path, codec).await
        }
        /// 删除/恢复文章
        pub async fn toggle_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::ToggleTopicRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ToggleTopicReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/topic.TopicService/ToggleTopic",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("topic.TopicService", "ToggleTopic"));
            self.inner.unary(req, path, codec).await
        }
        /// 获取文章详情
        pub async fn get_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTopicRequest>,
        ) -> std::result::Result<tonic::Response<super::GetTopicReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/topic.TopicService/GetTopic",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("topic.TopicService", "GetTopic"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod topic_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with TopicServiceServer.
    #[async_trait]
    pub trait TopicService: Send + Sync + 'static {
        /// 创建文章
        async fn create_topic(
            &self,
            request: tonic::Request<super::CreateTopicRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTopicReply>,
            tonic::Status,
        >;
        /// 修改文章
        async fn edit_topic(
            &self,
            request: tonic::Request<super::EditTopicRequest>,
        ) -> std::result::Result<tonic::Response<super::EditTopicReply>, tonic::Status>;
        /// 文章列表
        async fn list_topic(
            &self,
            request: tonic::Request<super::ListTopicRequest>,
        ) -> std::result::Result<tonic::Response<super::ListTopicReply>, tonic::Status>;
        /// 删除/恢复文章
        async fn toggle_topic(
            &self,
            request: tonic::Request<super::ToggleTopicRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ToggleTopicReply>,
            tonic::Status,
        >;
        /// 获取文章详情
        async fn get_topic(
            &self,
            request: tonic::Request<super::GetTopicRequest>,
        ) -> std::result::Result<tonic::Response<super::GetTopicReply>, tonic::Status>;
    }
    /// -- 在“文章”上定义rpc服务接口
    #[derive(Debug)]
    pub struct TopicServiceServer<T: TopicService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TopicService> TopicServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TopicServiceServer<T>
    where
        T: TopicService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/topic.TopicService/CreateTopic" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTopicSvc<T: TopicService>(pub Arc<T>);
                    impl<
                        T: TopicService,
                    > tonic::server::UnaryService<super::CreateTopicRequest>
                    for CreateTopicSvc<T> {
                        type Response = super::CreateTopicReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTopicRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TopicService>::create_topic(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateTopicSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/topic.TopicService/EditTopic" => {
                    #[allow(non_camel_case_types)]
                    struct EditTopicSvc<T: TopicService>(pub Arc<T>);
                    impl<
                        T: TopicService,
                    > tonic::server::UnaryService<super::EditTopicRequest>
                    for EditTopicSvc<T> {
                        type Response = super::EditTopicReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EditTopicRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TopicService>::edit_topic(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditTopicSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/topic.TopicService/ListTopic" => {
                    #[allow(non_camel_case_types)]
                    struct ListTopicSvc<T: TopicService>(pub Arc<T>);
                    impl<
                        T: TopicService,
                    > tonic::server::UnaryService<super::ListTopicRequest>
                    for ListTopicSvc<T> {
                        type Response = super::ListTopicReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTopicRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TopicService>::list_topic(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListTopicSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/topic.TopicService/ToggleTopic" => {
                    #[allow(non_camel_case_types)]
                    struct ToggleTopicSvc<T: TopicService>(pub Arc<T>);
                    impl<
                        T: TopicService,
                    > tonic::server::UnaryService<super::ToggleTopicRequest>
                    for ToggleTopicSvc<T> {
                        type Response = super::ToggleTopicReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ToggleTopicRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TopicService>::toggle_topic(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ToggleTopicSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/topic.TopicService/GetTopic" => {
                    #[allow(non_camel_case_types)]
                    struct GetTopicSvc<T: TopicService>(pub Arc<T>);
                    impl<
                        T: TopicService,
                    > tonic::server::UnaryService<super::GetTopicRequest>
                    for GetTopicSvc<T> {
                        type Response = super::GetTopicReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTopicRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TopicService>::get_topic(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTopicSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: TopicService> Clone for TopicServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: TopicService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TopicService> tonic::server::NamedService for TopicServiceServer<T> {
        const NAME: &'static str = "topic.TopicService";
    }
}
