use convert_case::{Case, Casing};
use desc::{FileDef, RpcDef, ServiceDef};
use quote::__private::TokenStream;
use quote::{format_ident, quote};

use crate::filegen::{rustify_name, CodeGenerator};

impl CodeGenerator<'_> {
    pub fn generate_server(&self, file: &FileDef, svc: &ServiceDef) -> TokenStream {
        let svc_qualified_raw_name = format!("{}.{}", file.package, svc.name);

        let svc_name = format_ident!("{}", rustify_name(svc.name.as_str()));
        let mod_name = format_ident!("{}_server", rustify_name(svc.name.as_str()));
        let server_name = format_ident!("{}Server", rustify_name(svc.name.as_str()));

        let mut trait_items = vec![];
        let mut arms = vec![];
        let mut defs = vec![];

        for (_, rpc) in &svc.rpc {
            let rpc: &RpcDef = rpc;
            let rpc_struct = format_ident!("{}Svc", rpc.name.as_str());
            let method_name = format_ident!("{}", rpc.name.as_str().to_case(Case::Snake));
            let stream_name = format_ident!("{}Stream", rpc.name.as_str());
            let path = format!("/{}.{}/{}", file.package, svc.name, rpc.name);

            let raw_req_type = self
                .base_type(&rpc.req_typ)
                // .with_context(|| format!("{msg_name}.{field_raw_name} in {:?}", file.name))
                .expect("Resolving name");

            let raw_res_type = self
                .base_type(&rpc.res_typ)
                // .with_context(|| format!("{msg_name}.{field_raw_name} in {:?}", file.name))
                .expect("Resolving name");

            let mut rpc_kind_method = quote! { unary };

            let mut req_type = raw_req_type.clone();
            let res_type;
            let response_type;
            let mut stream_def = quote! {};

            let svc_type = match (&rpc.req_stream, &rpc.res_stream) {
                (false, false) => {
                    req_type = quote! { super::#raw_req_type };
                    res_type = quote! { super::#raw_res_type };
                    response_type = quote! { Self::Response };
                    quote! { UnaryService }
                }
                (true, false) => {
                    req_type = quote! { tonic::Streaming<super::#raw_req_type> };
                    res_type = quote! { super::#raw_res_type };
                    response_type = quote! { Self::Response };
                    rpc_kind_method = quote! { client_streaming };
                    quote! { ClientStreamingService }
                }
                (false, true) => {
                    req_type = quote! { super::#req_type };
                    res_type = quote! { Self::#stream_name };
                    stream_def = quote! {
                        type ResponseStream = S::#stream_name;
                    };
                    trait_items.push(quote! {
                        type #stream_name: Stream<Item=Result<super::#raw_res_type, Status>> + Send + 'static;
                    });
                    response_type = quote! { Self::ResponseStream };
                    rpc_kind_method = quote! { server_streaming };
                    quote! { ServerStreamingService }
                }
                (true, true) => {
                    req_type = quote! { tonic::Streaming<super::#req_type> };
                    res_type = quote! { Self::#stream_name };
                    stream_def = quote! {
                        type ResponseStream = S::#stream_name;
                    };
                    trait_items.push(quote! {
                        type #stream_name: Stream<Item=Result<super::#raw_res_type, Status>> + Send + 'static;
                    });
                    response_type = quote! { Self::ResponseStream };
                    rpc_kind_method = quote! { streaming };
                    quote! { StreamingService }
                }
            };

            trait_items.push(quote! {
                async fn #method_name(&self, req: tonic::Request<#req_type>) -> Result<tonic::Response<#res_type>, tonic::Status>;
            });

            defs.push(quote! {
                struct #rpc_struct<S: #svc_name>(Arc<S>);
                impl<S: #svc_name> tonic::server::#svc_type<super::#raw_req_type> for #rpc_struct<S> {
                    type Response = super::#raw_res_type;
                    #stream_def
                    type Future = BoxFuture<
                        tonic::Response<#response_type>,
                        tonic::Status,
                    >;

                    fn call(&mut self, request: tonic::Request<#req_type>) -> Self::Future {
                        let inner = self.0.clone();
                        Box::pin(async move { inner.#method_name(request).await })
                    }
                }
            });
            arms.push(quote! {
                #path => {
                    let inner = self.0.clone();
                    let fut = async move {
                        let method = #rpc_struct(inner);
                        let codec = protokit::grpc::TonicCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.#rpc_kind_method(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
            });
        }
        quote! {
            mod #mod_name {
                use super::*;
                use protokit::grpc::*;
                #[protokit::grpc::async_trait]
                pub trait #svc_name : Send + Sync + 'static {
                    #(#trait_items)*
                }
                #[derive(Debug)]
                pub struct #server_name<S: #svc_name> (pub Arc<S>);
                impl<S: #svc_name> Clone for #server_name<S> {
                    fn clone(&self) -> Self {
                        Self(self.0.clone())
                    }
                }
                impl<S: #svc_name> From<S> for #server_name<S> {
                    fn from(v: S) -> Self {
                        Self(::std::sync::Arc::new(v))
                    }
                }
                impl<S : #svc_name> From<::std::sync::Arc<S>> for #server_name<S> {
                    fn from(v: ::std::sync::Arc<S>) -> Self {
                        Self(v)
                    }
                }

                #(#defs)*

                impl<S, B> Service<http::Request<B>> for #server_name<S>
                    where
                          S: #svc_name,
                          B: Body + Send + 'static,
                          B::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>> + Send + 'static,

                {
                    type Response = http::Response<tonic::body::BoxBody>;
                    type Error = core::convert::Infallible;
                    type Future = BoxFuture<Self::Response, Self::Error>;

                    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
                        Poll::Ready(Ok(()))
                    }

                    fn call(&mut self, req: http::Request<B>) -> Self::Future {
                        match req.uri().path() {
                            #(#arms)*
                            _ => Box::pin(async move {
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
                impl<S: #svc_name> tonic::server::NamedService for #server_name<S> {
                    const NAME: &'static str = #svc_qualified_raw_name;
                }
            }
            pub use #mod_name::*;
        }
    }

    pub fn generate_client(&self, file: &FileDef, svc: &ServiceDef) -> TokenStream {
        let _svc_qualified_raw_name = format!("{}.{}", file.package, svc.name);

        let _svc_name = format_ident!("{}", rustify_name(svc.name.as_str()));
        let mod_name = format_ident!("{}_client", rustify_name(svc.name.as_str()));
        let client_name = format_ident!("{}Client", rustify_name(svc.name.as_str()));

        let mut methods = vec![];
        for (_, rpc) in &svc.rpc {
            let rpc: &RpcDef = rpc;
            // let rpc_struct = format_ident!("{}Svc", rpc.name.as_str());
            let method_name = format_ident!("{}", rpc.name.as_str().to_case(Case::Snake));
            let _stream_name = format_ident!("{}Stream", rpc.name.as_str());
            let path = format!("/{}.{}/{}", file.package, svc.name, rpc.name);

            let raw_req_type = self
                .base_type(&rpc.req_typ)
                // .with_context(|| format!("{msg_name}.{field_raw_name} in {:?}", file.name))
                .expect("Resolving name");

            let raw_res_type = self
                .base_type(&rpc.res_typ)
                // .with_context(|| format!("{msg_name}.{field_raw_name} in {:?}", file.name))
                .expect("Resolving name");

            let _req_type = raw_req_type.clone();
            let _res_type = raw_res_type.clone();

            let mut rpc_kind_method = quote! { unary };

            let res_type = if rpc.res_stream {
                quote!( tonic::Streaming<super::#raw_res_type> )
            } else {
                quote! { super::#raw_res_type }
            };

            let (req_type, req_convert) = if rpc.req_stream {
                (
                    quote! { impl tonic::IntoStreamingRequest<Message=super::#raw_req_type> },
                    quote! { into_streaming_request },
                )
            } else {
                (
                    quote! { impl tonic::IntoRequest<super::#raw_req_type> },
                    quote! { into_request },
                )
            };

            match (&rpc.req_stream, &rpc.res_stream) {
                (false, false) => {}
                (true, false) => {
                    rpc_kind_method = quote! { client_streaming };
                }
                (false, true) => {
                    rpc_kind_method = quote! { server_streaming };
                }
                (true, true) => {
                    rpc_kind_method = quote! { streaming };
                }
            };
            methods.push(quote! {
                pub async fn #method_name(
                    &mut self,
                    request: #req_type,
                ) -> Result<tonic::Response<#res_type>, tonic::Status> {
                    self.inner
                        .ready()
                        .await
                        .map_err(|e| {
                            Status::new(Code::Unknown, format!("Service was not ready: {}", e.into()))
                        })?;
                    let codec = protokit
                    ::grpc::TonicCodec::default();
                    let path = http::uri::PathAndQuery::from_static(#path);
                    self.inner.#rpc_kind_method(request.#req_convert(), path, codec).await
                }
            })
        }
        quote! {
            mod #mod_name {
                use super::*;
                use protokit::grpc::*;
                #[derive(Debug, Clone)]
                pub struct #client_name<C> {
                    inner: tonic::client::Grpc<C>,
                }
                impl #client_name<tonic::transport::Channel> {
                    pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
                    where
                        D: core::convert::TryInto<tonic::transport::Endpoint>,
                        D::Error: Into<StdError>,
                    {
                        let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
                        Ok(Self::new(conn))
                    }
                }
                impl<S> #client_name<S>
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
                    ) -> #client_name<InterceptedService<S, F>>
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
                        #client_name::new(InterceptedService::new(inner, interceptor))
                    }
                    #(#methods)*
                }

            }
            pub use #mod_name::*;
        }
    }
}
