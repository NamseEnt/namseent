use namui::{Url, Uuid};

pub fn get_project_image_url(
    project_id: Uuid,
    image_id: Uuid,
) -> Result<Url, Box<dyn std::error::Error>> {
    let url_string = crate::append_slash![
        crate::SETTING.resource_base_url,
        "projects",
        project_id,
        "images",
        image_id,
    ];
    let url = Url::parse(&url_string)?;
    Ok(url)
}

#[allow(dead_code)]
pub fn get_character_main_image_url(character_id: Uuid) -> Result<Url, Box<dyn std::error::Error>> {
    let url_string = crate::append_slash![
        crate::SETTING.resource_base_url,
        "character",
        character_id,
        "main_image",
    ];
    let url = Url::parse(&url_string)?;
    Ok(url)
}

pub fn get_project_cg_part_variant_image_url(
    project_id: Uuid,
    cg_id: Uuid,
    part_variant_id: Uuid,
) -> Result<Url, Box<dyn std::error::Error>> {
    let file_name = format!("{part_variant_id}.webp");
    let url_string = crate::append_slash![
        crate::SETTING.resource_base_url,
        project_id,
        "cg",
        cg_id,
        file_name,
    ];
    let url = Url::parse(&url_string)?;
    Ok(url)
}

pub fn get_project_cg_thumbnail_image_url(
    project_id: Uuid,
    cg_id: Uuid,
) -> Result<Url, Box<dyn std::error::Error>> {
    let url_string = crate::append_slash![
        crate::SETTING.resource_base_url,
        project_id,
        "cg",
        cg_id,
        "thumbnail.webp",
    ];
    let url = Url::parse(&url_string)?;
    Ok(url)
}

#[macro_export]
macro_rules! append_slash {
    ($($x:expr),+ $(,)?) => {{
        let mut result = String::new();
        $(
            let x = $x.to_string();
            if result.is_empty() {
                result = x;
            } else if result.ends_with('/') {
                if let Some(x) = x.strip_prefix('/') {
                    result.push_str(x);
                } else  {
                    result.push_str(&x);
                }
            } else {
                if x.starts_with('/') {
                    result.push_str(&x);
                } else {
                    result.push('/');
                    result.push_str(&x);
                }
            }
        )+
        result
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_append_slash() {
        assert_eq!(append_slash!["a", "b"], "a/b");
        assert_eq!(append_slash!["a/", "b"], "a/b");
        assert_eq!(append_slash!["a", "/b"], "a/b");
        assert_eq!(append_slash!["a/", "/b"], "a/b");
        assert_eq!(append_slash!["a/", "/b/"], "a/b/");
        assert_eq!(append_slash!["a/", "b/"], "a/b/");
    }
}
