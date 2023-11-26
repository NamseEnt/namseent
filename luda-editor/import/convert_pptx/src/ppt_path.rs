const URL_PREFIX: &str = "https://example.com/";

pub fn convert_ppt_path_to_url(ppt_image_path: &str) -> String {
    format!("{URL_PREFIX}{}", ppt_image_path)
}

pub fn convert_url_to_ppt_path(url: &str) -> String {
    url.trim_start_matches(URL_PREFIX).to_string()
}
