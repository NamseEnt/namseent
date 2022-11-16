macro_rules! define_rpc {
    {
        $($service:ident: {
            $($method:ident: {
                $(pub struct $struct_name:ident $struct_type_block:tt)*
                $(pub enum $enum_name:ident $enum_type_block:tt)*
                Error $err_type_block:tt
            },)*
        },)*
    } => {
        // Rpc structs
        $($(
            pub mod $method {
                $(
                    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
                    pub struct $struct_name $struct_type_block
                )*

                $(
                    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
                    pub enum $enum_name $enum_type_block
                )*

                #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
                pub enum Error $err_type_block
                simple_error_impl!(Error);

                pub type Result = core::result::Result<Response, Error>;
            }
        )*)*


        #[cfg(feature = "server")]
        mod server {
            pub use hyper;
            $(
                pub trait $service<TSession> {
                    $(
                        fn $method<'a>(
                            &'a self,
                            session: Option<TSession>,
                            req: super::$method::Request,
                        ) -> std::pin::Pin<
                            Box<dyn 'a + std::future::Future<Output = super::$method::Result> + Send>,
                        >;
                    )*
                }
            )*

            pub async fn handle_rpc<'a, TSession>(
                request: hyper::Request<hyper::Body>,
                response_builder: hyper::http::response::Builder,
                $(#[allow(non_snake_case)] $service: &impl $service<TSession>,)*
                session: Option<TSession>,
            ) -> Result<hyper::Response<hyper::Body>, Box<dyn std::error::Error + Send + Sync>> {
                let query = request.uri().query();
                if query.is_none() {
                    return Ok(response_builder
                        .status(hyper::StatusCode::BAD_REQUEST)
                        .body(hyper::Body::from("No query"))
                        .unwrap());
                }
                let query = query.unwrap().to_string();

                let body = hyper::body::to_bytes(request.into_body()).await;
                if let Err(error) = body {
                    return Ok(response_builder
                        .status(hyper::StatusCode::BAD_REQUEST)
                        .body(hyper::Body::from(error.to_string()))
                        .unwrap());
                }
                let body = body.unwrap();

                match query {
                    $(
                        $(
                            query if query == stringify!($method) => {
                                let request = serde_json::from_slice::<super::$method::Request>(&body);
                                if let Err(error) = request {
                                    return Ok(response_builder
                                        .status(hyper::StatusCode::BAD_REQUEST)
                                        .body(hyper::Body::from(error.to_string()))
                                        .unwrap());
                                }
                                let request = request.unwrap();
                                let response = $service.$method(session, request).await;
                                let body = serde_json::to_string(&response).unwrap();
                                Ok(response_builder
                                    .status(hyper::StatusCode::OK)
                                    .body(hyper::Body::from(body))
                                    .unwrap())
                            }
                        )*
                    )*
                    _ => {
                        return Ok(response_builder
                            .status(hyper::StatusCode::BAD_REQUEST)
                            .body(hyper::Body::from("Unknown query"))
                            .unwrap());
                    }
                }
            }
        }
        #[cfg(feature = "server")]
        pub use server::*;

        #[cfg(feature = "client")]
        mod client {
            use std::sync::Mutex;

            pub struct RpcSetting {
                endpoint: String,
                session_id: Option<uuid::Uuid>,
            }

            pub struct Rpc {
                setting: Mutex<RpcSetting>,
            }

            impl Rpc {
                pub const fn new(endpoint: String) -> Self {
                    Self {
                        setting: Mutex::new(RpcSetting {
                            endpoint,
                            session_id: None,
                        }),
                    }
                }
                pub fn set_session_id(&self, session_id: uuid::Uuid) {
                    let mut setting = self.setting.lock().unwrap();
                    setting.session_id.replace(session_id);
                }
                pub fn session_id(&self) -> Option<uuid::Uuid> {
                    let setting = self.setting.lock().unwrap();
                    setting.session_id
                }
                pub fn set_endpoint(&self, endpoint: String) {
                    let mut setting = self.setting.lock().unwrap();
                    setting.endpoint = endpoint;
                }
                pub fn endpoint(&self) -> String {
                    let setting = self.setting.lock().unwrap();
                    setting.endpoint.clone()
                }
            }

            $($(
            impl Rpc {
                pub fn $method<'a>(
                    &'a self,
                    req: super::$method::Request,
                ) -> $crate::RpcFuture<super::$method::Result> {
                    pub async fn call<'a>(
                        endpoint: String,
                        session_id: Option<namui::Uuid>,
                        req: super::$method::Request,
                    ) -> super::$method::Result {
                        let url = format!("{endpoint}/?{method}", method = stringify!($method),);
                        let result = namui::network::http::fetch_json::<super::$method::Result>(
                            url,
                            namui::network::http::Method::POST,
                            |builder| {
                                let builder = builder
                                    .header("Content-Type", "application/json")
                                    .header("Accept", "application/json");
                                (if let Some(session_id) = session_id {
                                    builder.header("session", session_id.to_string())
                                } else {
                                    builder
                                })
                                .body(serde_json::to_string(&req).unwrap())
                            },
                        )
                        .await;

                        match result {
                            Ok(result) => result,
                            Err(error) => Err(super::$method::Error::Unknown(error.to_string())),
                        }
                    }
                    $crate::RpcFuture {
                        future: Box::new(Box::pin(call(self.endpoint(), self.session_id(), req))),
                    }
                }
            }
            )*)*
        }
        #[cfg(feature = "client")]
        pub use client::*;
    };
}

pub(crate) use define_rpc;

pub struct RpcFuture<RpcResult: 'static> {
    pub(crate) future: Box<dyn std::future::Future<Output = RpcResult> + Unpin + 'static>,
}

#[cfg(feature = "client")]
impl<RpcResult: 'static> RpcFuture<RpcResult> {
    pub fn callback(self, callback: impl FnOnce(RpcResult) + 'static) {
        let future = self.future;
        namui::spawn_local(async move {
            let result = future.await;
            callback(result);
        });
    }
}
impl<RpcResult: 'static> std::future::Future for RpcFuture<RpcResult> {
    type Output = RpcResult;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        match std::pin::Pin::new(&mut this.future).poll(cx) {
            std::task::Poll::Ready(result) => std::task::Poll::Ready(result),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}