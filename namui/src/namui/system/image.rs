use super::*;
use crate::{url::url_to_bytes, *};

pub(super) async fn init() -> InitResult {
    Ok(())
}

pub type Load<T> = Option<Result<T>>;

impl RenderCtx {
    pub fn image<'a>(&'a self, url: &Url) -> Sig<'a, Load<Image>> {
        let (load, set_load) = self.state(|| Load::None);

        self.effect(&format!("Load image from {url}"), || {
            let url = url.clone();
            spawn_local(async move {
                let image = load_image(&ImageSource::Url { url }).await;
                set_load.set(Some(image));
            });
        });

        load
    }
}

pub async fn load_image(image_source: &ImageSource) -> Result<Image> {
    match image_source {
        ImageSource::Url { url } => {
            let bytes = url_to_bytes(&url).await?;

            let image_bitmap = image_bitmap_from_u8(&bytes).await?;

            let wh = Wh::new(
                (image_bitmap.width() as f32).px(),
                (image_bitmap.height() as f32).px(),
            );

            system::drawer::load_image(image_source, image_bitmap);

            Ok(Image {
                src: image_source.clone(),
                wh,
            })
        }
    }
}

async fn image_bitmap_from_u8(data: &[u8]) -> Result<web_sys::ImageBitmap> {
    let u8_array = js_sys::Uint8Array::from(data);

    let u8_array_sequence = {
        let array = js_sys::Array::new();
        array.push(&u8_array);
        array
    };
    let blob = web_sys::Blob::new_with_u8_array_sequence(&u8_array_sequence.into()).unwrap();

    Ok(wasm_bindgen_futures::JsFuture::from(
        web_sys::window()
            .unwrap()
            .create_image_bitmap_with_blob(&blob)
            .unwrap(),
    )
    .await
    .unwrap()
    .into())
}
