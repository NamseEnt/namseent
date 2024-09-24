use crate::{server_connection, simple_button, toast};
use luda_rpc::{asset::reserve_team_asset_upload, AssetKind, AssetTag};
use namui::*;
use namui_prebuilt::table::*;
use network::http;
use psd_sprite::encode_psd_sprite;
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
                    let Ok(encoded_bytes) = encode_psd_sprite(&bytes) else {
                        toast::negative("에셋 인코딩 실패");
                        return;
                    };
                    match server_connection()
                        .reserve_team_asset_upload(RefRequest {
                            team_id: &team_id,
                            asset_name: &name,
                            byte_size: encoded_bytes.len() as u64,
                            asset_kind: &AssetKind::Sprite,
                            tags: &vec![AssetTag::System {
                                tag: luda_rpc::AssetSystemTag::SpriteCharacter,
                            }],
                        })
                        .await
                    {
                        Ok(Response {
                            presigned_put_uri,
                            headers,
                            ..
                        }) => match upload_asset(presigned_put_uri, headers, encoded_bytes).await {
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
    );

    // See protocol in select_asset_file.js
    let name = try_read_file_name(&mut data_rx).await?;
    let bytes = try_read_file_bytes(&mut data_rx).await?;

    drop(js_handle);

    return Ok(SelectedAssetFile { name, bytes });

    async fn try_read_file_name(rx: &mut UnboundedReceiver<Vec<u8>>) -> Result<String> {
        let name_bytes = rx.recv().await.ok_or(anyhow!("data channel closed"))?;
        if name_bytes.is_empty() {
            return Err(anyhow!("file not selected"));
        }
        Ok(String::from_utf8(name_bytes)?)
    }
    async fn try_read_file_bytes(rx: &mut UnboundedReceiver<Vec<u8>>) -> Result<Vec<u8>> {
        let file_bytes = rx.recv().await.ok_or(anyhow!("data channel closed"))?;
        Ok(file_bytes)
    }
}

async fn upload_asset(
    presigned_put_uri: String,
    headers: Vec<(String, String)>,
    bytes: Vec<u8>,
) -> Result<()> {
    let mut builder = http::Request::put(presigned_put_uri);
    for (key, value) in headers {
        builder = builder.header(key, value);
    }
    let response = builder.body(bytes)?.send().await?;
    let status = response.ensure_status_code()?.status();
    if !status.is_success() {
        return Err(anyhow!("status code: {}", status));
    }
    Ok(())
}
