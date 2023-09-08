use super::*;
use ::futures::future::try_join_all;
use namui::Uuid;

#[allow(dead_code)]
pub async fn upload_images_using_picker(
    project_id: Uuid,
) -> Result<(), Box<dyn std::error::Error>> {
    let files = namui::file::picker::open().await;

    let concurrency = 10;
    let file_count_per_channel = (files.len() as f32 / concurrency as f32).ceil() as usize;

    let futures = files
        .chunks(file_count_per_channel)
        .map(|files_in_channel| async move {
            for file in files_in_channel.iter() {
                let data = file.content().await;

                create_image(project_id, data.to_vec()).await?;
            }
            Result::<(), Box<dyn std::error::Error>>::Ok(())
        });

    try_join_all(futures).await?;

    Ok(())
}
