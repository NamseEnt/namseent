use namui::*;

#[derive(Debug, Clone, PartialEq, State)]
pub struct StickerImage {
    pub image: Image,
    pub wh: Wh<Px>,
    pub stroke_px: Px,
}

impl StickerImage {
    pub fn new(image: Image, wh: Wh<Px>, stroke_px: Px) -> Self {
        Self {
            image,
            wh,
            stroke_px,
        }
    }

    fn image_filter(&self) -> ImageFilter {
        let total_radius = Xy::new(
            OrderedFloat::new(self.stroke_px.as_f32()),
            OrderedFloat::new(self.stroke_px.as_f32()),
        );
        let inner_radius = Xy::new(
            OrderedFloat::new((self.stroke_px * 0.4).as_f32()),
            OrderedFloat::new((self.stroke_px * 0.4).as_f32()),
        );

        let source = ImageFilter::Image { src: self.image };
        let dilated_total = source.clone().dilate(total_radius, None);
        let dilated_inner = source.clone().dilate(inner_radius, None);

        let black_ring = ImageFilter::blend(
            BlendMode::DstOut,
            dilated_inner.clone().color_filter(ColorFilter::Blend {
                color: Color::BLACK,
                blend_mode: BlendMode::SrcIn,
            }),
            source.clone(),
        );

        let white_ring = ImageFilter::blend(
            BlendMode::DstOut,
            dilated_total.color_filter(ColorFilter::Blend {
                color: Color::WHITE,
                blend_mode: BlendMode::SrcIn,
            }),
            dilated_inner.clone().color_filter(ColorFilter::Blend {
                color: Color::WHITE,
                blend_mode: BlendMode::SrcIn,
            }),
        );

        let black_and_source = ImageFilter::blend(BlendMode::SrcOver, black_ring, source);
        let filter = ImageFilter::blend(BlendMode::SrcOver, white_ring, black_and_source);

        let image_wh = self.image.info().wh();
        let image_width = image_wh.width.as_f32();
        let image_height = image_wh.height.as_f32();
        let target_width = self.wh.width.as_f32();
        let target_height = self.wh.height.as_f32();
        let target_ratio = target_width / target_height;
        let image_ratio = image_width / image_height;

        let dest_rect = if image_ratio == target_ratio {
            Rect::from_xy_wh(Xy::zero(), self.wh)
        } else if image_ratio > target_ratio {
            let scale = target_width / image_width;
            let height = px(image_height * scale);
            let y = (self.wh.height - height) / 2.0;
            Rect::from_xy_wh(Xy::new(0.px(), y), Wh::new(self.wh.width, height))
        } else {
            let scale = target_height / image_height;
            let width = px(image_width * scale);
            let x = (self.wh.width - width) / 2.0;
            Rect::from_xy_wh(Xy::new(x, 0.px()), Wh::new(width, self.wh.height))
        };

        let scale_x = dest_rect.width().as_f32() / image_width;
        let scale_y = dest_rect.height().as_f32() / image_height;

        filter.with_local_matrix(
            TransformMatrix::from_translate(dest_rect.x().as_f32(), dest_rect.y().as_f32())
                * TransformMatrix::from_scale(scale_x, scale_y),
        )
    }
}

impl Component for StickerImage {
    fn render(self, ctx: &RenderCtx) {
        let paint = Paint::new(Color::WHITE).set_image_filter(self.image_filter());

        ctx.add(namui::image(ImageParam {
            rect: Rect::from_xy_wh(Xy::zero(), self.wh),
            image: self.image,
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint: Some(paint),
            },
        }));
    }
}
