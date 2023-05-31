use url::Url;

pub(crate) async fn url_to_bytes(url: &Url) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match url.scheme() {
        "http" | "https" => crate::network::http::get_bytes(url.as_str())
            .await
            .map_err(|error| error.into())
            .map(|bytes| bytes.as_ref().to_vec()),
        "bundle" => crate::file::bundle::read(url)
            .await
            .map_err(|error| error.into())
            .map(|bytes| bytes.as_ref().to_vec()),
        _ => Err(format!("unknown scheme: {}", url.scheme()).into()),
    }
}
