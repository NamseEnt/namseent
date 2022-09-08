use namui::Url;

pub fn get_character_image_url(
    character_id: &str,
    face_expression_id: &str,
) -> Result<Url, Box<dyn std::error::Error>> {
    let base_url = Url::parse(&crate::SETTING.resource_base_url)?;
    let url = base_url.join(&format!(
        "character/{character_id}/face_expression/{face_expression_id}"
    ))?;
    Ok(url)
}

pub fn get_character_main_image_url(character_id: &str) -> Result<Url, Box<dyn std::error::Error>> {
    let base_url = Url::parse(&crate::SETTING.resource_base_url)?;
    let url = base_url.join(&format!("character/{character_id}/main_image"))?;
    Ok(url)
}
