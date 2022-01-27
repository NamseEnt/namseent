use crate::app::sequence_list::{
    common::{render_button_text, render_rounded_rectangle, RoundedRectangleColor},
    events::SequenceListEvent,
    list::list_item::render_list_item,
    types::{
        RenderingTreeRow, RenderingTreeRows, SequenceLoadStateMap, SequenceTitlesLoadState,
        SequenceTitlesLoadStateDetail,
    },
    BUTTON_HEIGHT, MARGIN, SPACING,
};
use namui::{render, RenderingTree, Wh};
use num::clamp;

const SCROLL_BAR_WIDTH: f32 = MARGIN * 2.0;

pub fn render_list(
    wh: Wh<f32>,
    sequence_titles_load_state: &SequenceTitlesLoadState,
    sequence_load_state_map: &SequenceLoadStateMap,
    scroll_y: f32,
) -> RenderingTree {
    let inner_wh = Wh {
        width: wh.width - 2.0 * MARGIN - SPACING - SCROLL_BAR_WIDTH,
        height: wh.height - 2.0 * MARGIN,
    };
    let button_wh = Wh {
        width: inner_wh.width,
        height: BUTTON_HEIGHT,
    };
    let list_items: Vec<RenderingTreeRow> = match &sequence_titles_load_state.detail {
        SequenceTitlesLoadStateDetail::Loading => {
            vec![RenderingTreeRow {
                rendering_tree: render_button_text(button_wh, "Loading...".to_string()),
                height: button_wh.height,
            }]
        }
        SequenceTitlesLoadStateDetail::Loaded { titles } => titles
            .iter()
            .map(|title| {
                let path = format!("sequence/{}", title);
                let sequence_load_state = sequence_load_state_map.get(&path);
                render_list_item(inner_wh.width, title, &path, sequence_load_state)
            })
            .collect(),
        SequenceTitlesLoadStateDetail::Failed { error } => {
            vec![RenderingTreeRow {
                rendering_tree: render_button_text(button_wh, format!("Error: {}", error)),
                height: button_wh.height,
            }]
        }
    };
    let list_items_height = list_items.height(SPACING);
    let max_scroll_y = match list_items_height > inner_wh.height {
        true => list_items_height - inner_wh.height,
        false => 0.0,
    };
    let clamped_scroll_y = clamp(scroll_y, 0.0, max_scroll_y);
    let scroll_bar_height = inner_wh.height * inner_wh.height / (inner_wh.height + max_scroll_y);
    let scroll_bar_y = (inner_wh.height - scroll_bar_height)
        * match max_scroll_y == 0.0 {
            true => 0.0,
            false => clamped_scroll_y / max_scroll_y,
        };

    render![
        render_rounded_rectangle(wh, RoundedRectangleColor::Gray).attach_event(move |builder| {
            let scroll_y = clamped_scroll_y;
            builder.on_wheel(move |event| {
                let delta_y = event.delta_xy.y;
                let next_scroll_y = clamp(scroll_y + delta_y, 0.0, max_scroll_y);
                namui::event::send(SequenceListEvent::ScrolledEvent {
                    scroll_y: next_scroll_y,
                });
            })
        }),
        namui::clip(
            namui::PathBuilder::new().add_rect(&namui::LtrbRect {
                left: 0.0,
                top: 0.0,
                right: wh.width,
                bottom: wh.height,
            }),
            namui::ClipOp::Intersect,
            namui::translate(
                MARGIN,
                MARGIN - clamped_scroll_y,
                list_items.render(SPACING),
            )
        ),
        namui::translate(
            wh.width - MARGIN - SCROLL_BAR_WIDTH,
            MARGIN + scroll_bar_y,
            render_rounded_rectangle(
                Wh {
                    width: SCROLL_BAR_WIDTH,
                    height: scroll_bar_height
                },
                RoundedRectangleColor::LightGray,
            )
        ),
    ]
}
