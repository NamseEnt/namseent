use anyhow::Result;
use namui_skia::*;
use namui_type::*;
use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
};

pub(super) async fn init_skia() -> Result<Arc<Mutex<impl SkSkia + Send + Sync>>> {
    namui_skia::init_skia(
        crate::system::screen::window_id(),
        crate::system::screen::size(),
    )
}

pub(crate) fn on_window_resize(wh: Wh<IntPx>) {
    let mut skia = super::SKIA.get().unwrap().lock().unwrap();
    skia.on_resize(wh);
}

pub(crate) fn load_image(image_source: &ImageSource, bytes: &[u8]) -> ImageInfo {
    let skia = super::SKIA.get().unwrap().lock().unwrap();
    skia.load_image(image_source, bytes)
}

#[derive(Debug, Clone)]
pub(crate) struct ImageHandle {}

pub(crate) fn load_image2(bytes: &[u8], wh: Wh<usize>, color_type: ColorType) -> ImageHandle {
    todo!()
}

pub(crate) fn render(draw_input: DrawInput) {
    let mut skia = super::SKIA.get().unwrap().lock().unwrap();

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
