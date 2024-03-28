#[cfg(not(target_family = "wasm"))]
mod non_wasm;
#[cfg(target_family = "wasm")]
mod wasm;

use crate::{system::InitResult, url::url_to_bytes, *};
#[cfg(not(target_family = "wasm"))]
pub use non_wasm::*;
#[cfg(target_family = "wasm")]
pub use wasm::*;

pub(super) async fn init() -> InitResult {
    Ok(())
}

pub type Load<T> = Option<Result<T>>;

// impl RenderCtx {
//     pub fn image<'a>(&'a self, url: &Url) -> Sig<'a, Load<Image>> {
//         let url = self.track_eq(url);
//         let (url_load_tuple, set_load) = self.state(|| ((*url).clone(), Load::None));

//         self.effect(format!("Load image from {url}"), || {
//             let url = (*url).clone();

//             spawn(async move {
//                 let image = load_image(&ImageSource::Url {
//                     url: url.to_string(),
//                 })
//                 .await;
//                 set_load.mutate(move |x| {
//                     if x.0 != url {
//                         return;
//                     }
//                     x.1 = Some(image);
//                 });
//             });
//         });

//         url_load_tuple.map_1()
//     }
// }

pub async fn load_image(image_source: &ImageSource) -> Result<Image> {
    match image_source {
        ImageSource::Url { url } => {
            let bytes = url_to_bytes(&Url::parse(url)?).await?;

            let image_bitmap = ImageBitmap::from_u8(image_source, &bytes).await?;

            let wh = Wh::new(image_bitmap.width(), image_bitmap.height());

            system::drawer::load_image(image_source, image_bitmap);

            Ok(Image {
                src: image_source.clone(),
                wh,
            })
        }
        ImageSource::ImageHandle { image_handle: _ } => unreachable!(),
    }
}
