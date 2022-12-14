#[cfg(feature = "server")]
mod server {
    pub use hyper;
    pub trait AuthService<TSession> {
        fn exchange_google_auth_code_to_access_token<'a>(
            &'a self,
            session: Option<TSession>,
            req: super::exchange_google_auth_code_to_access_token::Request,
        ) -> std::pin::Pin<
            Box<
                dyn 'a
                    + std::future::Future<
                        Output = super::exchange_google_auth_code_to_access_token::Result,
                    >
                    + Send,
            >,
        >;
    }
    pub async fn handle_rpc<'a, TSession>(
        request: hyper::Request<hyper::Body>,
        response_builder: hyper::http::response::Builder,
        auth_service: &impl AuthService<TSession>,
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
            query if query == stringify!(exchange_google_auth_code_to_access_token) => {
                let request = serde_json::from_slice::<
                    super::AuthService::exchange_google_auth_code_to_access_token::Request,
                >(&body);
                if let Err(error) = request {
                    return Ok(response_builder
                        .status(hyper::StatusCode::BAD_REQUEST)
                        .body(hyper::Body::from(error.to_string()))
                        .unwrap());
                }
                let request = request.unwrap();
                let response = auth_service
                    .exchange_google_auth_code_to_access_token(session, request)
                    .await;
                let body = serde_json::to_string(&response).unwrap();
                Ok(response_builder
                    .status(hyper::StatusCode::OK)
                    .body(hyper::Body::from(body))
                    .unwrap())
            }
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
