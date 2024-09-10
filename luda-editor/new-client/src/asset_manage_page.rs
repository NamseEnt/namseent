use crate::{server_connection, simple_button, toast};
use luda_rpc::{asset::reserve_team_asset_upload, AssetKind};
use namui::*;
use namui_prebuilt::table::*;
use network::http;
use std::io::Write;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct AssetManagePage<'a> {
    pub team_id: &'a String,
}

impl Component for AssetManagePage<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { team_id } = self;

        let screen_wh = namui::screen::size().map(|x| x.into_px());

        let upload_asset = &|| {
            use reserve_team_asset_upload::*;
            ctx.spawn({
                let team_id = team_id.clone();
                async move {
                    let Ok(SelectedAssetFile { name, bytes }) = select_asset_file().await else {
                        toast::negative("에셋 파일 선택 실패");
                        return;
                    };
                    match server_connection()
                        .reserve_team_asset_upload(RefRequest {
                            team_id: &team_id,
                            asset_name: &name,
                            byte_size: bytes.len() as u64,
                            asset_kind: &AssetKind::Sprite,
                        })
                        .await
                    {
                        Ok(Response {
                            presigned_put_uri,
                            headers,
                            ..
                        }) => match upload_asset(presigned_put_uri, headers, bytes).await {
                            Ok(_) => toast::positive("에셋 업로드 성공".to_string()),
                            Err(_) => toast::negative("에셋 업로드 실패".to_string()),
                        },
                        Err(_error) => {
                            toast::negative("에셋 업로드 예약 실패".to_string());
                        }
                    };
                }
            });
        };

        ctx.compose(|ctx| {
            vertical([fixed(24.px(), |wh, ctx| {
                ctx.add(AssetUploadButton { wh, upload_asset });
            })])(screen_wh, ctx);
        });
    }
}

struct AssetUploadButton<'a> {
    wh: Wh<Px>,
    upload_asset: &'a dyn Fn(),
}
impl Component for AssetUploadButton<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, upload_asset } = self;

        ctx.add(simple_button(wh, "에셋 업로드", |_| {
            upload_asset();
        }));
    }
}

struct SelectedAssetFile {
    name: String,
    bytes: Vec<u8>,
}
async fn select_asset_file() -> Result<SelectedAssetFile> {
    let (data_tx, mut data_rx) = tokio::sync::mpsc::unbounded_channel();

    let js_handle = namui::wasi::insert_js(
        include_str!("select_asset_file.js"),
        Some(move |data: &[u8]| {
            data_tx.send(data.to_vec()).unwrap();
        }),
    )
    .await;

    // See protocol in select_asset_file.js
    let name = try_read_file_name(&mut data_rx).await?;
    let bytes = try_read_file_bytes(&mut data_rx).await?;

    drop(js_handle);

    return Ok(SelectedAssetFile { name, bytes });

    async fn try_read_i32(rx: &mut UnboundedReceiver<Vec<u8>>) -> Result<i32> {
        let bytes = rx.recv().await.ok_or(anyhow!("data channel closed"))?;
        if bytes.len() != 4 {
            return Err(anyhow!("invalid i32 bytes length: {}", bytes.len()));
        }
        let mut i32_bytes: [u8; 4] = [0; 4];
        i32_bytes.copy_from_slice(&bytes);
        Ok(i32::from_be_bytes(i32_bytes))
    }
    async fn try_read_file_name(rx: &mut UnboundedReceiver<Vec<u8>>) -> Result<String> {
        let name_byte_length = match try_read_i32(rx).await? {
            x if x > 0 => x,
            -1 => return Err(anyhow!("file name not selected")),
            x => return Err(anyhow!("invalid file name length: {x}")),
        };

        let name_bytes = rx.recv().await.ok_or(anyhow!("data channel closed"))?;
        if name_bytes.len() != name_byte_length as usize {
            return Err(anyhow!(
                "invalid file name bytes length: {}",
                name_bytes.len()
            ));
        }
        Ok(String::from_utf8(name_bytes)?)
    }
    async fn try_read_file_bytes(rx: &mut UnboundedReceiver<Vec<u8>>) -> Result<Vec<u8>> {
        let file_byte_length = try_read_i32(rx).await?;
        if file_byte_length < 0 {
            return Err(anyhow!("invalid file bytes length: {}", file_byte_length));
        }
        let mut file_bytes = Vec::with_capacity(file_byte_length as usize);
        let mut writer = std::io::Cursor::new(&mut file_bytes);
        let mut read_count = 0;

        while read_count < file_byte_length {
            let chunk_length = match try_read_i32(rx).await? {
                x if x > 0 => x,
                0 => return Err(anyhow!("chunk read aborted")),
                x => return Err(anyhow!("invalid chunk length: {x}")),
            };
            let chunk = rx.recv().await.ok_or(anyhow!("data channel closed"))?;
            if chunk.len() != chunk_length as usize {
                return Err(anyhow!(
                    "invalid chunk bytes length: {}, {} expected",
                    chunk.len(),
                    chunk_length
                ));
            }
            read_count += chunk_length;
            writer.write_all(&chunk)?;
        }
        Ok(file_bytes)
    }
}

async fn upload_asset(
    presigned_put_uri: String,
    headers: Vec<(String, String)>,
    bytes: Vec<u8>,
) -> Result<()> {
    let mut builder = http::Request::post(presigned_put_uri);
    for (key, value) in headers {
        builder = builder.header(key, value);
    }
    let response = builder.body(bytes)?.send().await?;
    response.ensure_status_code()?;
    Ok(())
}
