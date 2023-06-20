use super::*;
use namui::Uuid;

use std::path::Path;

pub async fn upload_images_using_picker(
    project_id: Uuid,
) -> Result<(), Box<dyn std::error::Error>> {
    let files = namui::file::picker::open().await;

    let concurrency = 10;
    let file_count_per_channel = (files.len() as f32 / concurrency as f32).ceil() as usize;

    let futures = files
        .chunks(file_count_per_channel)
        .map(|files_in_channel| async move {
            for file in files_in_channel.into_iter() {
                let filename = file.name();
                let _filename = Path::new(&filename)
                    .with_extension("")
                    .to_str()
                    .unwrap()
                    .to_string();

                let data = file.content().await;

                if let Err(error) = create_image(project_id, data.to_vec()).await {
                    return Err(error);
                }
            }
            Ok(())
        });

    futures::future::try_join_all(futures).await?;

    Ok(())
}
