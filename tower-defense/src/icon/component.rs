use crate::icon::Icon;
use namui::*;

impl Component for Icon {
    fn render(self, ctx: &RenderCtx) {
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
        let image = kind.image();

        // Create paint with opacity
        let paint = if opacity < 1.0 {
            Some(Paint::new(Color::from_f01(1.0, 1.0, 1.0, opacity)))
        } else {
            None
        };

        for attribute in attributes {
            let attribute_image = attribute.icon_kind.image();
            let attribute_render_rect = attribute.attribute_render_rect(rect);
            ctx.add(namui::image(ImageParam {
                rect: attribute_render_rect,
                image: attribute_image,
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: paint.clone(),
                },
            }));
        }
        ctx.add(namui::image(ImageParam {
            rect,
            image,
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint,
            },
        }));
    }
}
