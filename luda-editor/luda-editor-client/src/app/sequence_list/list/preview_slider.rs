use crate::app::sequence_list::{
    common::{render_rounded_rectangle, RoundedRectangleColor},
    events::SequenceListEvent,
    types::SequencePreviewProgressMap,
};
use namui::prelude::*;

pub fn render_preview_slider(
    wh: Wh<Px>,
    title: &String,
    sequence_preview_progress_map: &SequencePreviewProgressMap,
) -> RenderingTree {
    let thumb_wh = Wh {
        width: wh.height,
        height: wh.height,
    };
    let progress = sequence_preview_progress_map.get(title).unwrap_or(&0.0);
    let thumb_x = (wh.width - thumb_wh.width) * progress;

    render([
        render_rounded_rectangle(wh, RoundedRectangleColor::Gray).attach_event(move |builder| {
            let title = title.clone();
            builder.on_mouse_move_in(move |event| {
                let title = title.clone();
                let progress = (event.local_xy.x / wh.width).clamp(0.0, 1.0);
                namui::event::send(SequenceListEvent::PreviewSliderMovedEvent { title, progress });
            });
        }),
        namui::translate(
            thumb_x,
            px(0.0),
            render_rounded_rectangle(thumb_wh, RoundedRectangleColor::White),
        ),
    ])
}
