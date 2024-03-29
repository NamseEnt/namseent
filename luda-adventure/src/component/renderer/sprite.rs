use crate::app::game::{RenderingContext, Tile};
use namui::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Sprite {
    pub visual_rect: Rect<Tile>,
    pub image_url: Url,
}

impl Sprite {
    pub fn render(&self, rendering_context: &RenderingContext) -> RenderingTree {
        image(ImageParam {
            rect: px_rect(self.visual_rect, rendering_context),
            source: ImageSource::Url(self.image_url.clone()),
            style: ImageStyle {
                fit: ImageFit::Fill,
                paint_builder: None,
            },
        })
    }

    pub fn translate(&mut self, xy: Xy<Tile>) {
        self.visual_rect = self.visual_rect + xy;
    }
}

fn px_rect(tile_rect: Rect<Tile>, rendering_context: &RenderingContext) -> Rect<Px> {
    Rect::Xywh {
        x: (rendering_context.px_per_tile * tile_rect.x()).floor(),
        y: (rendering_context.px_per_tile * tile_rect.y()).floor(),
        width: (rendering_context.px_per_tile * tile_rect.width()).floor(),
        height: (rendering_context.px_per_tile * tile_rect.height()).floor(),
    }
}
