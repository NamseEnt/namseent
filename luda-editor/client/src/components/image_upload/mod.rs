use namui::prelude::*;
use rpc::data::*;
use rpc::utils::retry_on_error;

pub async fn create_image(
    project_id: namui::Uuid,
    image: Vec<u8>,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let image_id = {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&image);
        let hash = hasher.finalize().to_le_bytes();
        let bytes: [u8; 16] = [
            hash[0], hash[1], hash[2], hash[3], hash[0], hash[1], hash[2], hash[3], hash[0],
            hash[1], hash[2], hash[3], hash[0], hash[1], hash[2], hash[3],
        ];
        Uuid::from_bytes(bytes)
    };

    retry_on_error(
        move || {
            crate::RPC.put_image_meta_data(rpc::put_image_meta_data::Request {
                project_id,
                image_id,
                labels: Vec::new(),
            })
        },
        10,
    )
    .await?;

    let response = retry_on_error(
        move || {
            crate::RPC.prepare_upload_image(rpc::prepare_upload_image::Request {
                project_id,
                image_id,
            })
        },
        10,
    )
    .await?;

    let body = image;

    retry_on_error(
        move || {
            let body = body.clone();
            let upload_url = response.upload_url.clone();
            async move {
                namui::network::http::fetch(
                    upload_url,
                    namui::network::http::Method::PUT,
                    |builder| builder.body(body.to_vec()),
                )
                .await?
                .error_for_400599()
                .await
            }
        },
        10,
    )
    .await?;

    Ok(image_id)
}

#[allow(dead_code)]
pub async fn update_image(
    _prev_label_list: Vec<Label>,
    _new_label_list: Vec<Label>,
    _image: Option<File>,
) -> Result<(), Box<dyn std::error::Error>> {
    /*
    1) 이미지를 수정할 경우
    -> 그냥 put 하면 돼
    2) 레이블을 수정할 경우
    -> move 하면 돼
        -> 근데 move는 Copy & Delete야.
    3) 이미지와 레이블 둘 다 수정할 경우
    -> put & Delete

    실패할 수 있다. 실패했다고 알려주면 된다. Delete를 먼저 하진 않는다.
     */
    todo!()
}
