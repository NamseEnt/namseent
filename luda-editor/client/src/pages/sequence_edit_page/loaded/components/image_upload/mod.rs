mod upload_images;

use namui::file::picker::File;
use namui::prelude::*;
use rpc::data::*;
pub use upload_images::*;

pub async fn create_image(
    project_id: namui::Uuid,
    labels: Vec<Label>,
    image: Option<Box<[u8]>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let image_id = namui::uuid();

    retry_on_error(
        move || {
            crate::RPC.put_image_meta_data(rpc::put_image_meta_data::Request {
                project_id,
                image_id,
                labels: labels.clone(),
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

    let body = match image {
        Some(buffer) => buffer,
        None => [].into(),
    };

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

    Ok(())
}

async fn retry_on_error<FuncFuture, FuncOk, FuncErr>(
    func: impl Fn() -> FuncFuture,
    max_retry_count: usize,
) -> Result<FuncOk, FuncErr>
where
    FuncFuture: std::future::Future<Output = Result<FuncOk, FuncErr>>,
{
    let mut retry_count = 0;
    let mut delay = 100.ms();
    loop {
        match func().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if retry_count < max_retry_count {
                    retry_count += 1;
                    namui::time::delay(delay).await;
                    delay = {
                        let collision_avoidance = ((namui::random(1)[0] % 10) as f32).ms();
                        let next_delay = delay * 2 + collision_avoidance;
                        let max_delay = 4000.ms();
                        if next_delay > max_delay {
                            max_delay
                        } else {
                            next_delay
                        }
                    };
                    continue;
                } else {
                    return Err(error);
                }
            }
        }
    }
}

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
