use super::*;

macro_rules! simple_method_impl {
    ($method: expr, $method_name: ident, $method_bytes_name: ident, $method_serde_name: ident, $method_json_name: ident) => {
        pub async fn $method_name(url: impl IntoUrl) -> Result<Response, HttpError> {
            fetch(url, $method, |builder| builder).await
        }
        pub async fn $method_bytes_name(url: impl IntoUrl) -> Result<impl AsRef<[u8]>, HttpError> {
            fetch_bytes(url, $method, |builder| builder).await
        }
        pub async fn $method_serde_name<T, TDeserializeError, TDeserialize>(
            url: impl IntoUrl,
            deserialize: TDeserialize,
        ) -> Result<T, HttpError>
        where
            T: serde::de::DeserializeOwned,
            TDeserializeError: serde::de::Error,
            TDeserialize: FnOnce(&[u8]) -> Result<T, TDeserializeError>,
        {
            fetch_serde(url, $method, |builder| builder, deserialize).await
        }
        pub async fn $method_json_name<T: serde::de::DeserializeOwned>(
            url: impl IntoUrl,
        ) -> Result<T, HttpError> {
            fetch_json(url, $method, |builder| builder).await
        }
    };
}

simple_method_impl!(Method::GET, get, get_bytes, get_serde, get_json);
simple_method_impl!(Method::POST, post, post_bytes, post_serde, post_json);
simple_method_impl!(Method::PUT, put, put_bytes, put_serde, put_json);
simple_method_impl!(
    Method::DELETE,
    delete,
    delete_bytes,
    delete_serde,
    delete_json
);
simple_method_impl!(Method::PATCH, patch, patch_bytes, patch_serde, patch_json);
