#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(clippy::derive_partial_eq_without_eq)]
use std::fmt::Write;
use ::protokit::*;
use ::protokit as root;
use root::types::any::*;
use root::types::empty::*;
use root::types::any::*;
pub fn register_types(registry: &mut reflect::Registry) {
    registry.register(&Book::default());
    registry.register(&Book_Section::default());
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: Option<String>,
    pub x: Option<i32>,
    pub pack: Vec<i32>,
    pub pack2: Vec<f32>,
    pub category: Vec<Category>,
    pub sections: Vec<Book_Section>,
    pub test1: ::std::collections::HashMap<String, String>,
    pub other: Option<Box<Any>>,
    pub book: Option<Box<Book>>,
    pub extfield: String,
    pub id: BookOneOfId,
    pub _unknown: (),
}
impl Book {
    #[inline(always)]
    pub fn r#with_title(mut self, it: String) -> Self {
        self.r#set_title(it);
        self
    }
    #[inline(always)]
    pub fn r#set_title(&mut self, it: String) -> &mut Self {
        self.title = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_author(mut self, it: String) -> Self {
        self.r#set_author(it);
        self
    }
    #[inline(always)]
    pub fn r#set_author(&mut self, it: String) -> &mut Self {
        self.author = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_x(mut self, it: i32) -> Self {
        self.r#set_x(it);
        self
    }
    #[inline(always)]
    pub fn r#set_x(&mut self, it: i32) -> &mut Self {
        self.x = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_pack(mut self, it: i32) -> Self {
        self.r#add_pack(it);
        self
    }
    #[inline(always)]
    pub fn r#add_pack(&mut self, it: i32) -> &mut Self {
        self.pack.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_pack2(mut self, it: f32) -> Self {
        self.r#add_pack2(it);
        self
    }
    #[inline(always)]
    pub fn r#add_pack2(&mut self, it: f32) -> &mut Self {
        self.pack2.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_category(mut self, it: Category) -> Self {
        self.r#add_category(it);
        self
    }
    #[inline(always)]
    pub fn r#add_category(&mut self, it: Category) -> &mut Self {
        self.category.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_sections(mut self, it: Book_Section) -> Self {
        self.r#add_sections(it);
        self
    }
    #[inline(always)]
    pub fn r#add_sections(&mut self, it: Book_Section) -> &mut Self {
        self.sections.push(it);
        self
    }
    #[inline(always)]
    pub fn r#with_test1(mut self, k: String, v: String) -> Self {
        self.r#add_test1(k, v);
        self
    }
    #[inline(always)]
    pub fn r#add_test1(&mut self, k: String, v: String) -> &mut Self {
        let _ = self.test1.insert(k, v);
        self
    }
    #[inline(always)]
    pub fn r#with_other(mut self, it: Any) -> Self {
        self.r#set_other(it);
        self
    }
    #[inline(always)]
    pub fn r#set_other(&mut self, it: Any) -> &mut Self {
        self.other = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_book(mut self, it: Book) -> Self {
        self.r#set_book(it);
        self
    }
    #[inline(always)]
    pub fn r#set_book(&mut self, it: Book) -> &mut Self {
        self.book = Box::new(it).into();
        self
    }
    #[inline(always)]
    pub fn r#with_extfield(mut self, it: String) -> Self {
        self.r#set_extfield(it);
        self
    }
    #[inline(always)]
    pub fn r#set_extfield(&mut self, it: String) -> &mut Self {
        self.extfield = it.into();
        self
    }
    #[inline(always)]
    pub fn r#with_id_local(mut self, it: i32) -> Self {
        self.id = BookOneOfId::Local(it);
        self
    }
    #[inline(always)]
    pub fn r#set_id_local(&mut self, it: i32) -> &mut Self {
        self.id = BookOneOfId::Local(it);
        self
    }
    #[inline(always)]
    pub fn r#with_id_isbn(mut self, it: String) -> Self {
        self.id = BookOneOfId::Isbn(it);
        self
    }
    #[inline(always)]
    pub fn r#set_id_isbn(&mut self, it: String) -> &mut Self {
        self.id = BookOneOfId::Isbn(it);
        self
    }
}
impl textformat::Decodable for Book {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("title") => {
                textformat::Field::merge(&mut self.title, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("author") => {
                textformat::Field::merge(&mut self.author, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("x") => {
                textformat::Field::merge(&mut self.x, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("pack") => {
                textformat::Field::merge(&mut self.pack, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("pack2") => {
                textformat::Field::merge(&mut self.pack2, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("category") => {
                textformat::Field::merge(&mut self.category, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("sections") => {
                textformat::Field::merge(&mut self.sections, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("test1") => {
                textformat::Field::merge(&mut self.test1, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("other") => {
                textformat::Field::merge(&mut self.other, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("book") => {
                textformat::Field::merge(&mut self.book, ctx, value)?;
            }
            textformat::ast::FieldName::Extended("com.book.test1.extfield") => {
                textformat::Field::merge(&mut self.extfield, ctx, value)?;
            }
            textformat::ast::FieldName::Normal("local") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.id = BookOneOfId::Local(target);
            }
            textformat::ast::FieldName::Normal("isbn") => {
                let mut target = Default::default();
                textformat::Field::merge(&mut target, ctx, value)?;
                self.id = BookOneOfId::Isbn(target);
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Book {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.title != <String as Default>::default() {
            out.indent(pad);
            out.push_str("title: ");
            textformat::Field::format(&self.title, ctx, pad, out)?;
            out.push('\n');
        }
        if self.author != <Option<String> as Default>::default() {
            out.indent(pad);
            out.push_str("author: ");
            textformat::Field::format(&self.author, ctx, pad, out)?;
            out.push('\n');
        }
        if self.x != <Option<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("x: ");
            textformat::Field::format(&self.x, ctx, pad, out)?;
            out.push('\n');
        }
        if self.pack != <Vec<i32> as Default>::default() {
            out.indent(pad);
            out.push_str("pack: ");
            textformat::Field::format(&self.pack, ctx, pad, out)?;
            out.push('\n');
        }
        if self.pack2 != <Vec<f32> as Default>::default() {
            out.indent(pad);
            out.push_str("pack2: ");
            textformat::Field::format(&self.pack2, ctx, pad, out)?;
            out.push('\n');
        }
        if self.category != <Vec<Category> as Default>::default() {
            out.indent(pad);
            out.push_str("category: ");
            textformat::Field::format(&self.category, ctx, pad, out)?;
            out.push('\n');
        }
        if self.sections != <Vec<Book_Section> as Default>::default() {
            out.indent(pad);
            out.push_str("sections ");
            textformat::Field::format(&self.sections, ctx, pad, out)?;
            out.push('\n');
        }
        if self.test1
            != <::std::collections::HashMap<String, String> as Default>::default()
        {
            out.indent(pad);
            out.push_str("test1 ");
            textformat::Field::format(&self.test1, ctx, pad, out)?;
            out.push('\n');
        }
        if self.other != <Option<Box<Any>> as Default>::default() {
            out.indent(pad);
            out.push_str("other ");
            textformat::Field::format(&self.other, ctx, pad, out)?;
            out.push('\n');
        }
        if self.book != <Option<Box<Book>> as Default>::default() {
            out.indent(pad);
            out.push_str("book ");
            textformat::Field::format(&self.book, ctx, pad, out)?;
            out.push('\n');
        }
        if self.extfield != <String as Default>::default() {
            out.indent(pad);
            out.push_str("[com.book.test1.extfield]: ");
            textformat::Field::format(&self.extfield, ctx, pad, out)?;
            out.push('\n');
        }
        match &self.id {
            BookOneOfId::Local(value) => {
                out.indent(pad);
                out.push_str("local: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            BookOneOfId::Isbn(value) => {
                out.indent(pad);
                out.push_str("isbn: ");
                textformat::Field::format(value, ctx, pad, out)?;
                out.push('\n');
            }
            BookOneOfId::Unknown(..) => {}
        }
        Ok(())
    }
}
impl binformat::Decodable for Book {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            26u32 => {
                buf = Format::<Bytes>::decode(&mut self.title, buf)?;
            }
            34u32 => {
                buf = Format::<Bytes>::decode(&mut self.author, buf)?;
            }
            1848u32 => {
                buf = Format::<VInt>::decode(&mut self.x, buf)?;
            }
            1850u32 => {
                buf = Format::<VInt>::decode(&mut self.x, buf)?;
            }
            2584u32 => {
                buf = Format::<Repeat::<VInt>>::decode(&mut self.pack, buf)?;
            }
            2586u32 => {
                buf = Format::<Pack::<VInt>>::decode(&mut self.pack, buf)?;
            }
            25869u32 => {
                buf = Format::<Repeat::<Fix>>::decode(&mut self.pack2, buf)?;
            }
            25866u32 => {
                buf = Format::<Pack::<Fix>>::decode(&mut self.pack2, buf)?;
            }
            256u32 => {
                buf = Format::<Repeat::<Enum>>::decode(&mut self.category, buf)?;
            }
            258u32 => {
                buf = Format::<Repeat::<Enum>>::decode(&mut self.category, buf)?;
            }
            642u32 => {
                buf = Format::<Repeat::<Nest>>::decode(&mut self.sections, buf)?;
            }
            2578u32 => {
                buf = Format::<Map::<Bytes, Bytes>>::decode(&mut self.test1, buf)?;
            }
            282u32 => {
                buf = Format::<Nest>::decode(&mut self.other, buf)?;
            }
            2570570u32 => {
                buf = Format::<Nest>::decode(&mut self.book, buf)?;
            }
            530u32 => {
                buf = Format::<Bytes>::decode(&mut self.extfield, buf)?;
            }
            8u32 => {
                let mut tmp = Default::default();
                buf = Format::<VInt>::decode(&mut tmp, buf)?;
                self.id = BookOneOfId::Local(tmp);
            }
            10u32 => {
                let mut tmp = Default::default();
                buf = Format::<VInt>::decode(&mut tmp, buf)?;
                self.id = BookOneOfId::Local(tmp);
            }
            18u32 => {
                let mut tmp = Default::default();
                buf = Format::<Bytes>::decode(&mut tmp, buf)?;
                self.id = BookOneOfId::Isbn(tmp);
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Book {
    fn qualified_name(&self) -> &'static str {
        "com.book.test1.Book"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.title, 26u32, buf)?;
        Format::<Bytes>::encode(&self.author, 34u32, buf)?;
        Format::<VInt>::encode(&self.x, 1848u32, buf)?;
        Format::<Pack::<VInt>>::encode(&self.pack, 2586u32, buf)?;
        Format::<Pack::<Fix>>::encode(&self.pack2, 25866u32, buf)?;
        Format::<Repeat::<Enum>>::encode(&self.category, 258u32, buf)?;
        Format::<Repeat::<Nest>>::encode(&self.sections, 642u32, buf)?;
        Format::<Map::<Bytes, Bytes>>::encode(&self.test1, 2578u32, buf)?;
        Format::<Nest>::encode(&self.other, 282u32, buf)?;
        Format::<Nest>::encode(&self.book, 2570570u32, buf)?;
        Format::<Bytes>::encode(&self.extfield, 530u32, buf)?;
        match &self.id {
            BookOneOfId::Local(value) => {
                Format::<VInt>::encode(value, 8u32, buf)?;
            }
            BookOneOfId::Isbn(value) => {
                Format::<Bytes>::encode(value, 18u32, buf)?;
            }
            BookOneOfId::Unknown(..) => {}
        }
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[repr(C, u32)]
#[derive(Debug, Clone, PartialEq)]
pub enum BookOneOfId {
    Local(i32),
    Isbn(String),
    Unknown(::core::marker::PhantomData<()>),
}
impl Default for BookOneOfId {
    fn default() -> Self {
        BookOneOfId::Unknown(::core::marker::PhantomData)
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Book_Section {
    pub contents: String,
    pub _unknown: (),
}
impl Book_Section {
    #[inline(always)]
    pub fn r#with_contents(mut self, it: String) -> Self {
        self.r#set_contents(it);
        self
    }
    #[inline(always)]
    pub fn r#set_contents(&mut self, it: String) -> &mut Self {
        self.contents = it.into();
        self
    }
}
impl textformat::Decodable for Book_Section {
    fn merge_field(
        &mut self,
        ctx: &textformat::Context,
        name: &textformat::ast::FieldName,
        value: &textformat::ast::FieldValue,
    ) -> textformat::Result<()> {
        match name {
            textformat::ast::FieldName::Normal("contents") => {
                textformat::Field::merge(&mut self.contents, ctx, value)?;
            }
            other => textformat::bail!("{other:?} was not recognized"),
        }
        Ok(())
    }
}
impl textformat::Encodable for Book_Section {
    fn encode(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut std::string::String,
    ) -> textformat::Result<()> {
        if self.contents != <String as Default>::default() {
            out.indent(pad);
            out.push_str("contents: ");
            textformat::Field::format(&self.contents, ctx, pad, out)?;
            out.push('\n');
        }
        Ok(())
    }
}
impl binformat::Decodable for Book_Section {
    fn merge_field<'i, 'b>(
        &'i mut self,
        tag: u32,
        mut buf: binformat::ReadBuffer<'b>,
    ) -> binformat::Result<binformat::ReadBuffer<'b>> {
        use binformat::format::*;
        match tag {
            10u32 => {
                buf = Format::<Bytes>::decode(&mut self.contents, buf)?;
            }
            other => buf = self._unknown.merge_field(tag, buf)?,
        }
        Ok(buf)
    }
}
impl binformat::Encodable for Book_Section {
    fn qualified_name(&self) -> &'static str {
        "com.book.test1.Book.Section"
    }
    fn encode(&self, buf: &mut binformat::WriteBuffer) -> binformat::Result<()> {
        use binformat::format::*;
        Format::<Bytes>::encode(&self.contents, 10u32, buf)?;
        binformat::Encodable::encode(&self._unknown, buf)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Category {
    ADULT,
    CHILDREN,
    OTHER,
    Unknown(u32),
}
impl Default for Category {
    fn default() -> Category {
        Self::from(0)
    }
}
impl binformat::format::ProtoEnum for Category {}
impl From<u32> for Category {
    fn from(v: u32) -> Category {
        match v {
            0u32 => Category::ADULT,
            1u32 => Category::CHILDREN,
            2u32 => Category::OTHER,
            other => Category::Unknown(other),
        }
    }
}
impl From<Category> for u32 {
    fn from(v: Category) -> u32 {
        match v {
            Category::ADULT => 0u32,
            Category::CHILDREN => 1u32,
            Category::OTHER => 2u32,
            Category::Unknown(other) => other,
        }
    }
}
impl textformat::Field for Category {
    fn format(
        &self,
        ctx: &textformat::Context,
        pad: usize,
        out: &mut String,
    ) -> ::std::fmt::Result {
        let str = match self {
            Category::ADULT => "ADULT",
            Category::CHILDREN => "CHILDREN",
            Category::OTHER => "OTHER",
            Category::Unknown(n) => {
                write!(out, "{n}")?;
                return Ok(());
            }
        };
        out.push_str(str);
        Ok(())
    }
    fn merge_scalar(
        &mut self,
        _ctx: &textformat::Context,
        v: &textformat::ast::Literal,
    ) -> textformat::Result<()> {
        match v {
            textformat::ast::Literal::Identifier("ADULT") => *self = Category::ADULT,
            textformat::ast::Literal::Identifier("CHILDREN") => {
                *self = Category::CHILDREN;
            }
            textformat::ast::Literal::Identifier("OTHER") => *self = Category::OTHER,
            textformat::ast::Literal::Int(i) => *self = Self::from(*i as u32),
            other => textformat::bail!("Invalid enum value: {other:?}"),
        }
        Ok(())
    }
}
mod BookSvc_server {
    use super::root;
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
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.0.clone();
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
    use super::root;
    use protokit::grpc::*;
    #[derive(Debug, Clone)]
    pub struct BookSvcClient<C> {
        inner: tonic::client::Grpc<C>,
    }
    impl BookSvcClient<tonic::transport::Channel> {
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
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
