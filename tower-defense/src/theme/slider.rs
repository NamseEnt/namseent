use crate::theme::{
    palette,
    paper_container::{PaperContainerBackground, PaperTexture, PaperVariant},
};
use namui::*;

/// A simple pill-style slider.
///
/// - `value` is expected in [0.0, 1.0].
/// - `on_change` is called when the user clicks/drags the slider.
pub struct Slider<'a> {
    pub wh: Wh<Px>,
    pub value: f32,
    pub on_change: &'a dyn Fn(f32),
}

impl<'a> Slider<'a> {
    pub fn new(wh: Wh<Px>, value: f32, on_change: &'a dyn Fn(f32)) -> Self {
        Self {
            wh,
            value,
            on_change,
        }
    }
}

impl Component for Slider<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Slider {
            wh,
            value,
            on_change,
        } = self;

        let value = value.clamp(0.0, 1.0);
        let (dragging, set_dragging) = ctx.state(|| false);

        let thumb_diameter = wh.height * 1.25;
        let thumb_radius = thumb_diameter * 0.5;

        let track_inner_width = (wh.width - thumb_diameter).max(px(0.0));
        let thumb_center_x = thumb_radius + (track_inner_width.as_f32() * value).px();
        let thumb_center_y = wh.height * 0.5;

        ctx.translate((thumb_center_x - thumb_radius, thumb_center_y - thumb_radius))
            .add(PaperContainerBackground {
                width: thumb_diameter,
                height: thumb_diameter,
                texture: PaperTexture::Rough,
                variant: PaperVariant::Pill,
                color: palette::WHITE,
                shadow: false,
                arrow: None,
            });

        let fill_width = thumb_center_x;
        if fill_width > px(0.0) {
            ctx.add(PaperContainerBackground {
                width: fill_width,
                height: wh.height,
                texture: PaperTexture::Rough,
                variant: PaperVariant::Pill,
                color: palette::PRIMARY,
                shadow: false,
                arrow: None,
            });
        }

        ctx.add(PaperContainerBackground {
            width: wh.width,
            height: wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Pill,
            color: palette::SURFACE_CONTAINER_LOWEST,
            shadow: false,
            arrow: None,
        });

        let track_inner_width_f32 = track_inner_width.as_f32();
        let thumb_offset = thumb_radius.as_f32();
        let value_from_x =
            move |x: Px| ((x.as_f32() - thumb_offset) / track_inner_width_f32).clamp(0.0, 1.0);

        let handle_event = move |event: Event<'_>| match event {
            Event::MouseDown { event } => {
                if !event.is_local_xy_in() {
                    return;
                }
                event.stop_propagation();
                set_dragging.set(true);

                let x = event.local_xy().x;
                on_change(value_from_x(x));
            }
            Event::MouseMove { event } => {
                if !*dragging {
                    return;
                }
                let x = event.local_xy().x;
                on_change(value_from_x(x));
            }
            Event::MouseUp { event } => {
                if !*dragging {
                    return;
                }
                set_dragging.set(false);
                if !event.is_local_xy_in() {
                    return;
                }
                let x = event.local_xy().x;
                on_change(value_from_x(x));
            }
            _ => {}
        };

        ctx.attach_event(handle_event);
    }
}
