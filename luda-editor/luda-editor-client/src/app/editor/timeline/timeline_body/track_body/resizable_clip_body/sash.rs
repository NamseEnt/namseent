use crate::app::editor::TimelineRenderContext;
use namui::prelude::*;

const SASH_WIDTH: Px = px(15.0);
pub(super) struct SashBodyProps<'a> {
    pub context: &'a TimelineRenderContext<'a>,
    pub direction: SashDirection,
    pub clip_rect: Rect<Px>,
}
#[derive(Debug, Clone, Copy)]
pub enum SashDirection {
    Left,
    Right,
}
pub(super) fn render_sash(props: &SashBodyProps) -> RenderingTree {
    let sash_rect = get_sash_rect(props.clip_rect, props.direction);
    let path = PathBuilder::new().add_rect(sash_rect);

    let paint = PaintBuilder::new()
        .set_color(Color::from_u8(255, 127, 39, 255))
        .set_style(PaintStyle::Fill)
        .set_anti_alias(true);

    namui::path(path, paint)
}

pub(super) fn get_sash_rect(clip_rect: Rect<Px>, direction: SashDirection) -> Rect<Px> {
    match direction {
        SashDirection::Left => Rect::Xywh {
            x: clip_rect.x(),
            y: clip_rect.y(),
            width: SASH_WIDTH,
            height: clip_rect.height(),
        },
        SashDirection::Right => Rect::Xywh {
            x: clip_rect.x() + clip_rect.width() - SASH_WIDTH,
            y: clip_rect.y(),
            width: SASH_WIDTH,
            height: clip_rect.height(),
        },
    }
}
