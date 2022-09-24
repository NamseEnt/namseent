use namui::{Url, Uuid};

pub fn get_project_image_url(
    project_id: Uuid,
    image_id: Uuid,
) -> Result<Url, Box<dyn std::error::Error>> {
    let base_url = Url::parse(&crate::SETTING.resource_base_url)?;
    let url = base_url.join(&format!("projects/{project_id}/images/{image_id}"))?;
    Ok(url)
}

pub fn get_character_main_image_url(character_id: Uuid) -> Result<Url, Box<dyn std::error::Error>> {
    let base_url = Url::parse(&crate::SETTING.resource_base_url)?;
    let url = base_url.join(&format!("character/{character_id}/main_image"))?;
    Ok(url)
}
