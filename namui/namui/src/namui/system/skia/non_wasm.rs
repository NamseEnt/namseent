use anyhow::Result;
use namui_skia::*;
use namui_type::*;
use std::{
    ops::DerefMut,
    sync::{Arc, RwLock},
};

pub(super) async fn init_skia() -> Result<Arc<RwLock<impl SkSkia + Send + Sync>>> {
    namui_skia::init_skia(
        crate::system::screen::window_id(),
        crate::system::screen::size(),
    )
}

pub(crate) fn on_window_resize(wh: Wh<IntPx>) {
    let mut skia = super::SKIA.get().unwrap().write().unwrap();
    skia.on_resize(wh);
}

pub(crate) fn load_image(image_source: &ImageSource, bytes: &[u8]) -> ImageInfo {
    let skia = super::SKIA.get().unwrap().read().unwrap();
    skia.load_image(image_source, bytes)
}

pub(crate) fn load_image2(image_info: ImageInfo, bytes: &mut [u8]) -> ImageHandle {
    let mut skia = super::SKIA.get().unwrap().write().unwrap();
    skia.load_image_from_raw(image_info, bytes)
}

pub(crate) fn render(draw_input: DrawInput) {
    let mut skia = super::SKIA.get().unwrap().write().unwrap();

    namui_drawer_sys::draw(skia.deref_mut(), draw_input, &|image_source| {
        let image_source = image_source.clone();
        tokio::spawn(async move {
            crate::system::image::load_image(&image_source)
                .await
                .unwrap();

            crate::system::drawer::redraw();
        });
    });
}
