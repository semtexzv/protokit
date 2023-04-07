#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
use super::super::super::super::google::protobuf::any::*;
use super::super::super::super::google::protobuf::empty::*;
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Category(pub u32);
#[protoenum]
impl Category {
    #[var(0u32, "ADULT")]
    pub const ADULT: Category = Category(0u32);
    #[var(1u32, "CHILDREN")]
    pub const CHILDREN: Category = Category(1u32);
    #[var(2u32, "OTHER")]
    pub const OTHER: Category = Category(2u32);
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum BookOneOfId {
    #[field(1u32, "local", varint, raw)]
    Local(i32),
    #[field(2u32, "isbn", string, raw)]
    Isbn(String),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for BookOneOfId {
    fn default() -> Self {
        Self::Local(Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Book {
    #[field(3u32, "title", string, singular)]
    pub title: String,
    #[field(4u32, "author", string, optional)]
    pub author: Option<String>,
    #[field(231u32, "x", varint, optional)]
    pub x: Option<i32>,
    #[field(323u32, "pack", varint, packed)]
    pub pack: Vec<i32>,
    #[field(3233u32, "pack2", fixed32, packed)]
    pub pack2: Vec<f32>,
    #[field(32u32, "category", protoenum, packed)]
    pub category: Vec<Category>,
    #[field(80u32, "sections", nested, repeated)]
    pub sections: Vec<Book__Section>,
    #[field(322u32, "test1", map(string, string), singular)]
    pub test1: ::protokit::IndexMap<String, String>,
    #[field(35u32, "other", nested, optional)]
    pub other: Option<Box<Any>>,
    #[field(321321u32, "book", nested, optional)]
    pub book: Option<Box<Book>>,
    #[oneof([1u32, 2u32], ["local", "isbn"])]
    pub id: Option<BookOneOfId>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Book__Section {
    #[field(1u32, "contents", string, singular)]
    pub contents: String,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
mod BookSvc_server {
    use super::*;
    use protokit::grpc::*;
    #[protokit::grpc::async_trait]
    pub trait BookSvc: Send + Sync + 'static {
        async fn put_1(
            &self,
            req: tonic::Request<super::Book>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status>;
        async fn put_2(
            &self,
            req: tonic::Request<tonic::Streaming<super::Book>>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status>;
        type Put3Stream: Stream<Item = Result<super::Empty, Status>> + Send + 'static;
        async fn put_3(
            &self,
            req: tonic::Request<super::Book>,
        ) -> Result<tonic::Response<Self::Put3Stream>, tonic::Status>;
        type Put4Stream: Stream<Item = Result<super::Empty, Status>> + Send + 'static;
        async fn put_4(
            &self,
            req: tonic::Request<tonic::Streaming<super::Book>>,
        ) -> Result<tonic::Response<Self::Put4Stream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct BookSvcServer<S: BookSvc>(pub Arc<S>);
    impl<S: BookSvc> Clone for BookSvcServer<S> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<S: BookSvc> From<S> for BookSvcServer<S> {
        fn from(v: S) -> Self {
            Self(::std::sync::Arc::new(v))
        }
    }
    impl<S: BookSvc> From<::std::sync::Arc<S>> for BookSvcServer<S> {
        fn from(v: ::std::sync::Arc<S>) -> Self {
            Self(v)
        }
    }
    struct Put1Svc<S: BookSvc>(Arc<S>);
    impl<S: BookSvc> tonic::server::UnaryService<super::Book> for Put1Svc<S> {
        type Response = super::Empty;
        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
        fn call(&mut self, request: tonic::Request<super::Book>) -> Self::Future {
            let inner = self.0.clone();
            Box::pin(async move { inner.put_1(request).await })
        }
    }
    struct Put2Svc<S: BookSvc>(Arc<S>);
    impl<S: BookSvc> tonic::server::ClientStreamingService<super::Book> for Put2Svc<S> {
        type Response = super::Empty;
        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
        fn call(
            &mut self,
            request: tonic::Request<tonic::Streaming<super::Book>>,
        ) -> Self::Future {
            let inner = self.0.clone();
            Box::pin(async move { inner.put_2(request).await })
        }
    }
    struct Put3Svc<S: BookSvc>(Arc<S>);
    impl<S: BookSvc> tonic::server::ServerStreamingService<super::Book> for Put3Svc<S> {
        type Response = super::Empty;
        type ResponseStream = S::Put3Stream;
        type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
        fn call(&mut self, request: tonic::Request<super::Book>) -> Self::Future {
            let inner = self.0.clone();
            Box::pin(async move { inner.put_3(request).await })
        }
    }
    struct Put4Svc<S: BookSvc>(Arc<S>);
    impl<S: BookSvc> tonic::server::StreamingService<super::Book> for Put4Svc<S> {
        type Response = super::Empty;
        type ResponseStream = S::Put4Stream;
        type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
        fn call(
            &mut self,
            request: tonic::Request<tonic::Streaming<super::Book>>,
        ) -> Self::Future {
            let inner = self.0.clone();
            Box::pin(async move { inner.put_4(request).await })
        }
    }
    impl<S, B> Service<http::Request<B>> for BookSvcServer<S>
    where
        S: BookSvc,
        B: Body + Send + 'static,
        B::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>> + Send
            + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = core::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/com.book.test1.BookSvc/Put1" => {
                    let inner = self.0.clone();
                    let fut = async move {
                        let method = Put1Svc(inner);
                        let codec = ::protokit::grpc::TonicCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/com.book.test1.BookSvc/Put2" => {
                    let inner = self.0.clone();
                    let fut = async move {
                        let method = Put2Svc(inner);
                        let codec = ::protokit::grpc::TonicCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/com.book.test1.BookSvc/Put3" => {
                    let inner = self.0.clone();
                    let fut = async move {
                        let method = Put3Svc(inner);
                        let codec = ::protokit::grpc::TonicCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/com.book.test1.BookSvc/Put4" => {
                    let inner = self.0.clone();
                    let fut = async move {
                        let method = Put4Svc(inner);
                        let codec = ::protokit::grpc::TonicCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.streaming(method, req).await;
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
    impl<S: BookSvc> tonic::transport::NamedService for BookSvcServer<S> {
        const NAME: &'static str = "com.book.test1.BookSvc";
    }
}
pub use BookSvc_server::*;
mod BookSvc_client {
    use super::*;
    use protokit::grpc::*;
    #[derive(Debug, Clone)]
    pub struct BookSvcClient<C> {
        inner: tonic::client::Grpc<C>,
    }
    impl BookSvcClient<tonic::transport::Channel> {
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: core::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<S> BookSvcClient<S>
    where
        S: tonic::client::GrpcService<tonic::body::BoxBody>,
        S::Error: Into<StdError>,
        S::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <S::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: S) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: S,
            interceptor: F,
        ) -> BookSvcClient<InterceptedService<S, F>>
        where
            F: tonic::service::Interceptor,
            S::ResponseBody: Default,
            S: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <S as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <S as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            BookSvcClient::new(InterceptedService::new(inner, interceptor))
        }
        pub async fn put_1(
            &mut self,
            request: impl tonic::IntoRequest<super::Book>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    Status::new(
                        Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = ::protokit::grpc::TonicCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.book.test1.BookSvc/Put1",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn put_2(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::Book>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    Status::new(
                        Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = ::protokit::grpc::TonicCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.book.test1.BookSvc/Put2",
            );
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        pub async fn put_3(
            &mut self,
            request: impl tonic::IntoRequest<super::Book>,
        ) -> Result<tonic::Response<tonic::Streaming<super::Empty>>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    Status::new(
                        Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = ::protokit::grpc::TonicCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.book.test1.BookSvc/Put3",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn put_4(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::Book>,
        ) -> Result<tonic::Response<tonic::Streaming<super::Empty>>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    Status::new(
                        Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = ::protokit::grpc::TonicCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.book.test1.BookSvc/Put4",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
    }
}
pub use BookSvc_client::*;
