pub fn callback_url(authority: &str) -> String {
    let scheme = if authority.starts_with("localhost") || authority.starts_with("127.0.0.1") {
        "http"
    } else {
        "https"
    };
    format!("{scheme}://{authority}/oauth/github/callback")
}
