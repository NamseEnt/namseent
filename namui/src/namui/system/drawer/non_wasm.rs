use crate::{image::ImageBitmap, system::InitResult, *};

pub(crate) async fn init() -> InitResult {
    Ok(())
}

static mut LAST_RENDERING_TREE: Option<RenderingTree> = None;

pub(crate) fn request_draw_rendering_tree(rendering_tree: RenderingTree) {
    if let Some(last_rendering_tree) = unsafe { &mut LAST_RENDERING_TREE } {
        if last_rendering_tree == &rendering_tree {
            return;
        }
    }

    unsafe {
        LAST_RENDERING_TREE = Some(rendering_tree.clone());
    }

    let draw_input = DrawInput { rendering_tree };
    todo!()
}

pub(crate) fn load_typeface(typeface_name: &str, bytes: &[u8]) {
    // nothing
}

pub(crate) fn load_image(image_source: &ImageSource, image_bitmap: ImageBitmap) {}
