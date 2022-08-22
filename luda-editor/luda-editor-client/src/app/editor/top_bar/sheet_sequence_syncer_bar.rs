use crate::app::editor::sheet_sequence_syncer::{
    SheetSequenceSyncerEvent, SheetSequenceSyncerStatus,
};
use namui::prelude::*;

pub(super) struct SheetSequenceSyncerBarProps<'a> {
    pub height: Px,
    pub syncer_status: &'a SheetSequenceSyncerStatus,
}

/// NOTE: render_sheet_sequence_syncer_bar will be called after translating to be zero point for TopBar's right.
pub(super) fn render_sheet_sequence_syncer_bar(
    props: &SheetSequenceSyncerBarProps,
) -> RenderingTree {
    let status_text = match props.syncer_status {
        SheetSequenceSyncerStatus::Idle => None,
        SheetSequenceSyncerStatus::Syncing => Some("Syncing...".to_string()),
        SheetSequenceSyncerStatus::Failed(error) => Some(format!("Failed: {}", error)),
        SheetSequenceSyncerStatus::Successful => Some("Successfully synced".to_string()),
    };

    let button_width = props.height * 2.0;
    const MARGIN: Px = px(4.0);

    let status_text_rendering_tree = match status_text {
        Some(text_content) => namui::text(TextParam {
            x: -MARGIN,
            y: props.height / 2.0,
            align: TextAlign::Right,
            baseline: TextBaseline::Middle,
            font_type: FontType {
                font_weight: FontWeight::REGULAR,
                language: Language::Ko,
                serif: false,
                size: (props.height / 3.5 * 2.0).into(),
            },
            style: TextStyle {
                color: Color::BLACK,
                background: None,
                border: None,
                drop_shadow: None,
            },
            text: text_content,
        }),
        None => RenderingTree::Empty,
    };

    let button_rect = rect(RectParam {
        rect: Rect::Xywh {
            x: px(0.0),
            y: px(0.0),
            width: button_width,
            height: props.height,
        },
        style: RectStyle {
            stroke: Some(RectStroke {
                width: px(1.0),
                border_position: BorderPosition::Inside,
                color: Color::BLACK,
            }),
            fill: Some(RectFill {
                color: Color::WHITE,
            }),
            ..Default::default()
        },
    })
    .with_mouse_cursor(match props.syncer_status {
        SheetSequenceSyncerStatus::Idle
        | SheetSequenceSyncerStatus::Failed(_)
        | SheetSequenceSyncerStatus::Successful => namui::MouseCursor::Pointer,
        SheetSequenceSyncerStatus::Syncing => namui::MouseCursor::Default,
    })
    .attach_event(|builder| {
        builder.on_mouse_up_in(|_| {
            namui::event::send(SheetSequenceSyncerEvent::RequestSyncStart);
        });
    });
    let button_text = namui::text(TextParam {
        x: button_width / 2.0,
        y: props.height / 2.0,
        align: TextAlign::Center,
        baseline: TextBaseline::Middle,
        font_type: FontType {
            font_weight: FontWeight::REGULAR,
            language: Language::Ko,
            serif: false,
            size: (props.height / 3.0 * 2.0).into(),
        },
        style: TextStyle {
            color: Color::BLACK,
            background: None,
            border: None,
            drop_shadow: None,
        },
        text: "Sync".to_string(),
    });

    let button = render([button_rect, button_text]);

    translate(
        -button_width,
        px(0.0),
        render([status_text_rendering_tree, button]),
    )
}
