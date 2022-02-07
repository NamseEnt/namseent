use crate::app::editor::sequence_saver::SequenceSaverStatus;
use namui::prelude::*;

pub(super) struct SavingStatusTextProps<'a> {
    pub height: f32,
    pub sequence_saver_status: &'a SequenceSaverStatus,
}

pub(super) fn render_saving_status_text(props: &SavingStatusTextProps) -> RenderingTree {
    if *props.sequence_saver_status == SequenceSaverStatus::Idle {
        return RenderingTree::Empty;
    }

    text(TextParam {
        x: 0.0,
        y: props.height / 2.0,
        align: TextAlign::Left,
        baseline: TextBaseline::Middle,
        font_type: FontType {
            font_weight: FontWeight::REGULAR,
            language: Language::Ko,
            serif: false,
            size: (props.height / 3.0 * 2.0) as i16,
        },
        style: TextStyle {
            color: Color::BLACK,
            background: None,
            border: None,
            drop_shadow: None,
        },
        text: match &props.sequence_saver_status {
            SequenceSaverStatus::Idle => unreachable!(),
            SequenceSaverStatus::Saving => "Saving...".to_string(),
            SequenceSaverStatus::Saved => "Saved".to_string(),
            SequenceSaverStatus::Failed(error) => format!("Saving Failed: {}", error),
        },
    })
}
