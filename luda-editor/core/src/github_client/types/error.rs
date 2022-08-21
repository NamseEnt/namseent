#[derive(Debug)]
pub enum GithubClientError {
    NetworkError(namui::network::http::HttpError),
    BodySerializeError(Box<dyn std::error::Error>),
    ResponseParseError(Box<dyn std::error::Error>),
    CacheError(Box<dyn std::error::Error>),
}
namui::simple_error_impl!(GithubClientError);

impl From<namui::network::http::HttpError> for GithubClientError {
    fn from(error: namui::network::http::HttpError) -> Self {
        match error {
            namui::network::http::HttpError::Status { .. }
            | namui::network::http::HttpError::Timeout { .. }
            | namui::network::http::HttpError::Request { .. }
            | namui::network::http::HttpError::RedirectPolicy { .. }
            | namui::network::http::HttpError::Connection { .. }
            | namui::network::http::HttpError::Body { .. }
            | namui::network::http::HttpError::Unknown(_)
            | namui::network::http::HttpError::UrlParseError(_) => Self::NetworkError(error),
            namui::network::http::HttpError::Deserialize { .. }
            | namui::network::http::HttpError::JsonParseError(_)
            | namui::network::http::HttpError::Decode { .. }
            | namui::network::http::HttpError::TextParseError { .. } => {
                GithubClientError::ResponseParseError(error.into())
            }
        }
    }
}
