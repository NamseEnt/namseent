use crate::icon::Icon;
use namui::*;
use namui_prebuilt::simple_rect;

impl Icon {
    pub fn to_rendering_tree(&self) -> RenderingTree {
        self.to_rendering_tree_with_border(None)
    }

    pub fn to_rendering_tree_with_border(&self, border: Option<TextStyleBorder>) -> RenderingTree {
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

        // Add attribute images
        for attribute in attributes {
            let attribute_image = attribute.icon_kind.image();
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

        // Add main icon image with optional stroke
        let image = kind.image();
        let image_tree = if let Some(border) = border {
            render_icon_image_with_stroke(image, rect, *opacity, border)
        } else {
            let paint = if *opacity < 1.0 {
                Some(Paint::new(Color::from_f01(1.0, 1.0, 1.0, *opacity)))
            } else {
                None
            };
            namui::image(ImageParam {
                rect,
                image,
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint,
                },
            })
        };
        rendering_trees.push(image_tree);

        rendering_trees.push(simple_rect(
            *wh,
            Color::TRANSPARENT,
            0.px(),
            Color::TRANSPARENT,
        ));
        namui::render(rendering_trees)
    }
}

fn render_icon_image_with_stroke(
    image: Image,
    rect: Rect<Px>,
    opacity: f32,
    border: TextStyleBorder,
) -> RenderingTree {
    let paint_color = Color::WHITE.with_alpha((opacity * 255.0).round() as u8);
    let paint =
        Paint::new(paint_color).set_image_filter(icon_image_filter(image, rect.wh(), border));

    namui::image(ImageParam {
        rect,
        image,
        style: ImageStyle {
            fit: ImageFit::Contain,
            paint: Some(paint),
        },
    })
}

fn icon_image_filter(image: Image, width_height: Wh<Px>, border: TextStyleBorder) -> ImageFilter {
    let source = ImageFilter::Image { src: image };
    let image_wh = image.info().wh();
    let image_width = image_wh.width.as_f32();
    let image_height = image_wh.height.as_f32();
    let target_width = width_height.width.as_f32();
    let target_height = width_height.height.as_f32();
    let target_ratio = target_width / target_height;
    let image_ratio = image_width / image_height;

    let dest_rect = if image_ratio == target_ratio {
        Rect::from_xy_wh(Xy::zero(), width_height)
    } else if image_ratio > target_ratio {
        let scale = target_width / image_width;
        let height = px(image_height * scale);
        let y = (width_height.height - height) / 2.0;
        Rect::from_xy_wh(Xy::new(0.px(), y), Wh::new(width_height.width, height))
    } else {
        let scale = target_height / image_height;
        let width = px(image_width * scale);
        let x = (width_height.width - width) / 2.0;
        Rect::from_xy_wh(Xy::new(x, 0.px()), Wh::new(width, width_height.height))
    };

    let scale_x = dest_rect.width().as_f32() / image_width;
    let scale_y = dest_rect.height().as_f32() / image_height;
    let total_radius = Xy::new(
        OrderedFloat::new(border.width.as_f32() / scale_x),
        OrderedFloat::new(border.width.as_f32() / scale_y),
    );

    let dilated_total = source.clone().dilate(total_radius, None);
    let outer_color = dilated_total.color_filter(ColorFilter::Blend {
        color: border.color,
        blend_mode: BlendMode::SrcIn,
    });

    let border_ring = ImageFilter::blend(BlendMode::DstOut, outer_color, source.clone());
    let combined = ImageFilter::blend(BlendMode::SrcOver, border_ring, source.clone());

    combined.with_local_matrix(
        TransformMatrix::from_translate(dest_rect.x().as_f32(), dest_rect.y().as_f32())
            * TransformMatrix::from_scale(scale_x, scale_y),
    )
}
