use super::super::super::image_upload::create_image;
use namui::Uuid;
use rpc::data::Label;
use std::path::Path;

pub async fn upload_images(project_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
    let files = namui::file::picker::open().await;

    let concurrency = 10;
    let file_count_per_channel = (files.len() as f32 / concurrency as f32).ceil() as usize;

    let futures = files
        .chunks(file_count_per_channel)
        .map(|files_in_channel| async move {
            for file in files_in_channel.into_iter() {
                let filename = file.name();
                let filename = Path::new(&filename)
                    .with_extension("")
                    .to_str()
                    .unwrap()
                    .to_string();

                let labels: Vec<Label> = filename
                    .split('-')
                    .map(|splitted| {
                        let (key, value_with_sign) = splitted.split_at(splitted.find('=').unwrap());
                        let value = value_with_sign.split_at(1).1;
                        Label {
                            key: key.to_string(),
                            value: value.to_string(),
                        }
                    })
                    .collect();

                let data = file.content().await;

                if let Err(error) = create_image(project_id, labels, Some(data)).await {
                    return Err(error);
                }
            }
            Ok(())
        });

    futures::future::try_join_all(futures).await?;

    Ok(())
}
