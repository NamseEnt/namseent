use crate::{image::ImageBitmap, url::url_to_bytes};
use anyhow::Result;
use namui_hooks::*;
use namui_skia::*;
use namui_type::Wh;
use url::Url;

pub type Load<T> = Option<Result<T>>;

pub trait ImageTrait {
    fn image(&self, url: impl AsRef<str>) -> Sig<Load<Image>, &Load<Image>>;
}

impl ImageTrait for RenderCtx<'_, '_> {
    fn image(&self, url: impl AsRef<str>) -> Sig<Load<Image>, &Load<Image>> {
        let url = self.track_eq(&url.as_ref().to_string());
        let (load, set_load) = self.state(|| Load::None);

        self.effect(format!("Load image from {url}"), || {
            let url = (*url).clone();

            if load.is_some() {
                set_load.set(None);
                return EffectCleanUp::None;
            }

            let set_load = set_load.cloned();
            let join_handle = crate::spawn(async move {
                let image = load_image(&ImageSource::Url { url: url.clone() }).await;
                set_load.set(Some(image));
            });

            EffectCleanUp::once(move || {
                join_handle.abort();
            })
        });

        load
    }
}

pub async fn load_image(image_source: &ImageSource) -> Result<Image> {
    match image_source {
        ImageSource::Url { url } => {
            let bytes = url_to_bytes(&Url::parse(url)?).await?;

            let image_bitmap = ImageBitmap::from_u8(image_source, &bytes).await?;

            let wh = Wh::new(image_bitmap.width(), image_bitmap.height());

            crate::system::drawer::load_image(image_source, image_bitmap);

            Ok(Image {
                src: image_source.clone(),
                wh,
            })
        }
        ImageSource::ImageHandle { image_handle: _ } => unreachable!(),
    }
}
