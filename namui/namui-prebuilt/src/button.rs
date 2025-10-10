use crate::{simple_rect, typography::center_text_full_height};
use namui::*;
use std::borrow::Cow;

fn attach_text_button_event(
    ctx: ComposeCtx,
    mouse_buttons: Vec<MouseButton>,
    on_mouse_up_in: impl FnOnce(MouseEvent<'_>),
) {
    ctx.attach_event(|event| {
        if let Event::MouseUp { event } = event {
            if !event.is_local_xy_in() {
                return;
            }
            let Some(button) = event.button else {
                return;
            };
            if mouse_buttons.contains(&button) {
                on_mouse_up_in(event);
            }
        }
    });
}

pub struct TextButton<Text: AsRef<str>, OnMouseUpIn: FnOnce(MouseEvent)> {
    pub rect: Rect<Px>,
    pub text: Text,
    pub text_color: Color,
    pub stroke_color: Color,
    pub stroke_width: Px,
    pub fill_color: Color,
    pub mouse_buttons: Vec<MouseButton>,
    pub on_mouse_up_in: OnMouseUpIn,
}
impl<Text: AsRef<str>, OnMouseUpIn: FnOnce(MouseEvent)> Component
    for TextButton<Text, OnMouseUpIn>
{
    fn render(self, ctx: &RenderCtx) {
        let Self {
            rect,
            text,
            text_color,
            stroke_color,
            stroke_width,
            fill_color,
            mouse_buttons,
            on_mouse_up_in,
        } = self;
        ctx.compose(|ctx| {
            ctx.translate((rect.x(), rect.y()))
                .add(center_text_full_height(rect.wh(), text, text_color))
                .add(simple_rect(
                    rect.wh(),
                    stroke_color,
                    stroke_width,
                    fill_color,
                ));
            attach_text_button_event(ctx, mouse_buttons, on_mouse_up_in);
        });
    }
}

pub struct TextButtonFit<'a, Text>
where
    Text: Into<Cow<'a, str>>,
{
    pub height: Px,
    pub text: Text,
    pub text_color: Color,
    pub stroke_color: Color,
    pub stroke_width: Px,
    pub fill_color: Color,
    pub side_padding: Px,
    pub mouse_buttons: Vec<MouseButton>,
    pub on_mouse_up_in: &'a dyn Fn(MouseEvent),
}
impl<'a, Text> Component for TextButtonFit<'a, Text>
where
    Text: Into<Cow<'a, str>>,
{
    fn render(self, ctx: &RenderCtx) {
        let Self {
            height,
            text,
            text_color,
            stroke_color,
            stroke_width,
            fill_color,
            side_padding,
            mouse_buttons,
            on_mouse_up_in,
        } = self;
        let center_text = center_text_full_height(Wh::new(0.px(), height), text.into(), text_color);
        let width = center_text
            .bounding_box()
            .map(|bounding_box| bounding_box.width());

        ctx.compose(|ctx| {
            if let Some(width) = width {
                ctx.translate((width / 2 + side_padding, 0.px()))
                    .add(center_text);
                ctx.add(simple_rect(
                    Wh::new(width + side_padding * 2, height),
                    stroke_color,
                    stroke_width,
                    fill_color,
                ));
                attach_text_button_event(ctx, mouse_buttons, on_mouse_up_in);
            }
        });
    }
}

pub struct TextButtonFitAlign<'a> {
    pub wh: Wh<Px>,
    pub align: TextAlign,
    pub text: &'a str,
    pub text_color: Color,
    pub stroke_color: Color,
    pub stroke_width: Px,
    pub fill_color: Color,
    pub side_padding: Px,
    pub mouse_buttons: Vec<MouseButton>,
    pub on_mouse_up_in: &'a dyn Fn(MouseEvent),
}
impl Component for TextButtonFitAlign<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            align,
            text,
            text_color,
            stroke_color,
            stroke_width,
            fill_color,
            side_padding,
            mouse_buttons,
            on_mouse_up_in,
        } = self;
        let center_text = center_text_full_height(Wh::new(0.px(), wh.height), text, text_color);
        let center_text_width = center_text
            .bounding_box()
            .map(|bounding_box| bounding_box.width());

        ctx.compose(|ctx| {
            if let Some(center_text_width) = center_text_width {
                let center_text_x = (wh.width - center_text_width)
                    * match align {
                        TextAlign::Left => 0.0,
                        TextAlign::Center => 0.5,
                        TextAlign::Right => 1.0,
                    };

                ctx.translate((center_text_x, 0.px())).add(center_text);
                ctx.translate((center_text_x - center_text_width / 2 - side_padding, 0.px()))
                    .add(simple_rect(
                        Wh::new(center_text_width + side_padding * 2, wh.height),
                        stroke_color,
                        stroke_width,
                        fill_color,
                    ));

                attach_text_button_event(ctx, mouse_buttons, on_mouse_up_in);
            }
        });
    }
}

pub struct BodyTextButton<'a> {
    pub rect: Rect<Px>,
    pub text: &'a str,
    pub text_color: Color,
    pub stroke_color: Color,
    pub stroke_width: Px,
    pub fill_color: Color,
    pub text_align: TextAlign,
    pub mouse_buttons: Vec<MouseButton>,
    pub on_mouse_up_in: &'a dyn Fn(MouseEvent),
}
impl Component for BodyTextButton<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            rect,
            text,
            text_color,
            stroke_color,
            stroke_width,
            fill_color,
            text_align,
            mouse_buttons,
            on_mouse_up_in,
        } = self;

        ctx.compose(|ctx| {
            ctx.translate(rect.xy())
                .add(match text_align {
                    TextAlign::Left => {
                        crate::typography::body::left(rect.wh().height, text, text_color)
                    }
                    TextAlign::Center => {
                        crate::typography::body::center(rect.wh(), text, text_color)
                    }
                    TextAlign::Right => crate::typography::body::right(rect.wh(), text, text_color),
                })
                .add(simple_rect(
                    rect.wh(),
                    stroke_color,
                    stroke_width,
                    fill_color,
                ));

            attach_text_button_event(ctx, mouse_buttons, on_mouse_up_in);
        });
    }
}
