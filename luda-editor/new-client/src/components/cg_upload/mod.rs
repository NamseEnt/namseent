use rpc::{data::CgFile, utils::retry_on_error};

pub async fn create_cg(
    project_id: namui::Uuid,
    psd_file_name: String,
    psd_file: Vec<u8>,
) -> Result<CgFile, Box<dyn std::error::Error>> {
    let response = retry_on_error(
        {
            let psd_file_size = psd_file.len();
            let psd_file_name = psd_file_name.clone();
            move || {
                crate::RPC.request_put_psd_presigned_url(
                    rpc::request_put_psd_presigned_url::Request {
                        project_id,
                        psd_file_name: psd_file_name.clone(),
                        psd_file_size,
                    },
                )
            }
        },
        10,
    )
    .await?;

    let body = psd_file;

    retry_on_error(
        move || {
            let body = body.clone();
            let upload_url = response.presigned_url.clone();
            async move {
                namui::network::http::fetch(
                    upload_url,
                    namui::network::http::Method::PUT,
                    |builder| {
                        builder
                            .body(body.to_vec())
                            .header("content-type", "image/vnd.adobe.photoshop")
                    },
                )
                .await?
                .error_for_400599()
                .await
            }
        },
        10,
    )
    .await?;

    let rpc::complete_put_psd::Response { cg_id } = retry_on_error(
        {
            let psd_file_name = psd_file_name.clone();
            move || {
                let psd_file_name = psd_file_name.clone();
                crate::RPC.complete_put_psd(rpc::complete_put_psd::Request {
                    project_id,
                    psd_file_name,
                    psd_id: response.psd_id,
                })
            }
        },
        10,
    )
    .await?;

    let rpc::get_cg_file::Response { cg_file } = retry_on_error(
        move || crate::RPC.get_cg_file(rpc::get_cg_file::Request { cg_id, project_id }),
        10,
    )
    .await?;

    Ok(cg_file)
}
