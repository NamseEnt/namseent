use crate::pages::sequence_edit_page::atom::SEQUENCE_ATOM;
use namui::Uuid;
use rpc::data::{CutUpdateAction, ScreenGraphic};

pub fn send_to_back(screen_graphics: &[(Uuid, ScreenGraphic)], cut_id: Uuid, graphic_index: Uuid) {
    if let Some(last_graphic_index) = screen_graphics.last().map(|(index, _)| *index) {
        if last_graphic_index == graphic_index {
            return;
        }
        SEQUENCE_ATOM.mutate(move |sequence| {
            sequence.update_cut(
                cut_id,
                CutUpdateAction::ChangeGraphicOrder {
                    graphic_index,
                    after_graphic_index: Some(last_graphic_index),
                },
            )
        });
    }
}

pub fn send_backward(screen_graphics: &[(Uuid, ScreenGraphic)], cut_id: Uuid, graphic_index: Uuid) {
    let Some(next_or_last_graphic_index) = ({
        screen_graphics
            .iter()
            .position(|(index, _)| *index == graphic_index)
            .and_then(|position| {
                let next_position = (position + 1).min(screen_graphics.len() - 1);
                screen_graphics.get(next_position).map(|(index, _)| *index)
            })
    }) else {
        return;
    };
    if next_or_last_graphic_index == graphic_index {
        return;
    }

    SEQUENCE_ATOM.mutate(move |sequence| {
        sequence.update_cut(
            cut_id,
            CutUpdateAction::ChangeGraphicOrder {
                graphic_index,
                after_graphic_index: Some(next_or_last_graphic_index),
            },
        )
    });
}

pub fn bring_forward(screen_graphics: &[(Uuid, ScreenGraphic)], graphic_index: Uuid, cut_id: Uuid) {
    let previous_graphic_index = {
        screen_graphics
            .iter()
            .position(|(index, _)| *index == graphic_index)
            .and_then(|position| match position.checked_sub(2) {
                Some(position) => screen_graphics.get(position).map(|(index, _)| *index),
                None => None,
            })
    };
    if previous_graphic_index == Some(graphic_index) {
        return;
    }

    SEQUENCE_ATOM.mutate(move |sequence| {
        sequence.update_cut(
            cut_id,
            CutUpdateAction::ChangeGraphicOrder {
                graphic_index,
                after_graphic_index: previous_graphic_index,
            },
        )
    });
}

pub fn bring_to_front(
    screen_graphics: &[(Uuid, ScreenGraphic)],
    cut_id: Uuid,
    graphic_index: Uuid,
) {
    if screen_graphics.first().map(|(index, _)| *index) == Some(graphic_index) {
        return;
    }
    SEQUENCE_ATOM.mutate(move |sequence| {
        sequence.update_cut(
            cut_id,
            CutUpdateAction::ChangeGraphicOrder {
                graphic_index,
                after_graphic_index: None,
            },
        )
    });
}
