use crate::{asset_loader::icon_asset_loader::IconAssetLoader, icon::Icon};
use namui::*;
use namui_prebuilt::simple_rect;

impl Icon {
    pub fn to_rendering_tree(&self) -> RenderingTree {
        let Self {
            kind,
            size,
            attributes,
            wh,
            opacity,
        } = self;
        let icon_size = size.px();
        let icon_wh = Wh {
            width: icon_size,
            height: icon_size,
        };
        let icon_xy = Xy::new(
            (wh.width - icon_wh.width) / 2.0,
            (wh.height - icon_wh.height) / 2.0,
        );
        let rect = Rect::from_xy_wh(icon_xy, icon_wh);

        let mut rendering_trees = Vec::new();

        // Try to get images from global asset loader
        if let Some(global_loader) = IconAssetLoader::get_global() {
            // Add attribute images
            for attribute in attributes {
                if let Some(attribute_image) = global_loader.get(attribute.icon_kind) {
                    let attribute_render_rect = attribute.attribute_render_rect(rect);
                    let paint = if *opacity < 1.0 {
                        Some(Paint::new(Color::from_f01(1.0, 1.0, 1.0, *opacity)))
                    } else {
                        None
                    };
                    rendering_trees.push(namui::image(ImageParam {
                        rect: attribute_render_rect,
                        image: attribute_image,
                        style: ImageStyle {
                            fit: ImageFit::Contain,
                            paint: paint.clone(),
                        },
                    }));
                }
            }

            // Add main icon image
            if let Some(image) = global_loader.get(*kind) {
                let paint = if *opacity < 1.0 {
                    Some(Paint::new(Color::from_f01(1.0, 1.0, 1.0, *opacity)))
                } else {
                    None
                };
                rendering_trees.push(namui::image(ImageParam {
                    rect,
                    image,
                    style: ImageStyle {
                        fit: ImageFit::Contain,
                        paint,
                    },
                }));
            }
        }

        rendering_trees.push(simple_rect(
            *wh,
            Color::TRANSPARENT,
            0.px(),
            Color::TRANSPARENT,
        ));
        namui::render(rendering_trees)
    }
}
