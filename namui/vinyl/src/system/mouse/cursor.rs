use crate::*;

pub(crate) async fn load_default_cursor_set() -> Result<StandardCursorSpriteSet> {
    let sheet_path = "__system__/cursor/capitaine_24.png";
    let metadata_path = "__system__/cursor/capitaine_24.txt";

    let (sheet_image, metadata_bytes) = try_join!(
        system::image::load_image(ResourceLocation::bundle(sheet_path)),
        async move {
            file::bundle::read(metadata_path)
                .await
                .map_err(|e| e.into())
        },
    )?;

    let metadata_text = String::from_utf8_lossy(&metadata_bytes);

    StandardCursorSpriteSet::parse(sheet_image, metadata_text.as_ref())
}
