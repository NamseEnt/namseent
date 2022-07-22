use crate::app::sequence_list::{
    common::{render_button_text, render_rounded_rectangle, RoundedRectangleColor},
    events::SequenceListEvent,
    list::list_item::render_list_item,
    types::{
        RenderingTreeRow, RenderingTreeRows, SequenceOpenStateMap, SequencePreviewProgressMap,
        SequenceSyncState, SequencesSyncStateDetail,
    },
    BUTTON_HEIGHT, MARGIN, SPACING,
};
use namui::prelude::*;
use num::clamp;

pub fn render_list(
    wh: Wh<Px>,
    sequences_sync_state: &SequenceSyncState,
    sequence_preview_progress_map: &SequencePreviewProgressMap,
    scroll_y: Px,
    opened_sequence_title: &Option<String>,
    sequence_open_state_map: &SequenceOpenStateMap,
) -> RenderingTree {
    let scroll_bar_width = MARGIN * 2.0;

    let inner_wh = Wh {
        width: wh.width - 2.0 * MARGIN - SPACING - scroll_bar_width,
        height: wh.height - 2.0 * MARGIN,
    };
    let button_wh = Wh {
        width: inner_wh.width,
        height: BUTTON_HEIGHT,
    };
    let list_items: Vec<RenderingTreeRow> = match &sequences_sync_state.detail {
        SequencesSyncStateDetail::Loading => {
            vec![RenderingTreeRow {
                rendering_tree: render_button_text(button_wh, "Loading...".to_string()),
                height: button_wh.height,
            }]
        }
        SequencesSyncStateDetail::Loaded { title_sequence_map } => title_sequence_map
            .iter()
            .map(|(title, sequence)| {
                let is_item_opened = opened_sequence_title
                    .as_ref()
                    .map(|opened_title| title == opened_title)
                    .unwrap_or(false);
                let sequence_open_state = sequence_open_state_map.get(title);

                render_list_item(
                    inner_wh.width,
                    title,
                    sequence,
                    &sequence_preview_progress_map,
                    is_item_opened,
                    sequence_open_state,
                )
            })
            .collect(),
        SequencesSyncStateDetail::Failed { error } => {
            vec![RenderingTreeRow {
                rendering_tree: render_button_text(button_wh, format!("Error: {}", error)),
                height: button_wh.height,
            }]
        }
    };
    let list_items_height = list_items.height(SPACING);
    let max_scroll_y = match list_items_height > inner_wh.height {
        true => list_items_height - inner_wh.height,
        false => px(0.0),
    };
    let clamped_scroll_y = clamp(scroll_y, px(0.0), max_scroll_y);
    let scroll_bar_height = inner_wh.height * (inner_wh.height / (inner_wh.height + max_scroll_y));
    let scroll_bar_y = (inner_wh.height - scroll_bar_height)
        * match max_scroll_y == px(0.0) {
            true => 0.0,
            false => clamped_scroll_y / max_scroll_y,
        };

    render([
        render_rounded_rectangle(wh, RoundedRectangleColor::Gray).attach_event(move |builder| {
            let scroll_y = clamped_scroll_y;
            builder.on_wheel(move |event| {
                let delta_y_in_px = px(event.delta_xy.y);
                let next_scroll_y = clamp(scroll_y + delta_y_in_px, px(0.0), max_scroll_y);
                namui::event::send(SequenceListEvent::ScrolledEvent {
                    scroll_y: next_scroll_y,
                });
            });
        }),
        namui::clip(
            namui::PathBuilder::new().add_rect(namui::Rect::Ltrb {
                left: px(0.0),
                top: px(0.0),
                right: wh.width,
                bottom: wh.height,
            }),
            namui::ClipOp::Intersect,
            namui::translate(
                MARGIN,
                MARGIN - clamped_scroll_y,
                list_items.render(SPACING),
            ),
        ),
        namui::translate(
            wh.width - MARGIN - scroll_bar_width,
            MARGIN + scroll_bar_y,
            render_rounded_rectangle(
                Wh {
                    width: scroll_bar_width,
                    height: scroll_bar_height,
                },
                RoundedRectangleColor::LightGray,
            ),
        ),
    ])
}
