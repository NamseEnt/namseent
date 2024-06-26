use crate::pages::sequence_edit_page::atom::SEQUENCE_ATOM;
use namui::Uuid;
use rpc::data::{ChangeGraphicOrderAction, ScreenGraphic};

pub fn send_to_back(screen_graphics: &[(Uuid, ScreenGraphic)], cut_id: Uuid, graphic_index: Uuid) {
    if let Some(last_graphic_index) = screen_graphics.last().map(|(index, _)| *index) {
        if let Ok(change_graphic_order_action) =
            ChangeGraphicOrderAction::new(graphic_index, Some(last_graphic_index))
        {
            SEQUENCE_ATOM.mutate(move |sequence| {
                sequence.update_cut(cut_id, change_graphic_order_action.into())
            });
        };
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
    if let Ok(change_graphic_order_action) =
        ChangeGraphicOrderAction::new(graphic_index, Some(next_or_last_graphic_index))
    {
        SEQUENCE_ATOM.mutate(move |sequence| {
            sequence.update_cut(cut_id, change_graphic_order_action.into())
        });
    };
}

pub fn bring_forward(screen_graphics: &[(Uuid, ScreenGraphic)], cut_id: Uuid, graphic_index: Uuid) {
    let previous_graphic_index = {
        screen_graphics
            .iter()
            .position(|(index, _)| *index == graphic_index)
            .and_then(|position| match position.checked_sub(2) {
                Some(position) => screen_graphics.get(position).map(|(index, _)| *index),
                None => None,
            })
    };
    if let Ok(change_graphic_order_action) =
        ChangeGraphicOrderAction::new(graphic_index, previous_graphic_index)
    {
        SEQUENCE_ATOM.mutate(move |sequence| {
            sequence.update_cut(cut_id, change_graphic_order_action.into())
        });
    }
}

pub fn bring_to_front(
    screen_graphics: &[(Uuid, ScreenGraphic)],
    cut_id: Uuid,
    graphic_index: Uuid,
) {
    if screen_graphics.first().map(|(index, _)| *index) == Some(graphic_index) {
        return;
    }
    if let Ok(change_graphic_order_action) = ChangeGraphicOrderAction::new(graphic_index, None) {
        SEQUENCE_ATOM.mutate(move |sequence| {
            sequence.update_cut(cut_id, change_graphic_order_action.into())
        });
    }
}

pub enum Command {
    SendToBack,
    SendBackward,
    BringForward,
    BringToFront,
}

pub fn run_command(
    screen_graphics: &[(Uuid, ScreenGraphic)],
    cut_id: Uuid,
    graphic_index: Uuid,
    command: Command,
) {
    match command {
        Command::SendToBack => send_to_back(screen_graphics, cut_id, graphic_index),
        Command::SendBackward => send_backward(screen_graphics, cut_id, graphic_index),
        Command::BringForward => bring_forward(screen_graphics, cut_id, graphic_index),
        Command::BringToFront => bring_to_front(screen_graphics, cut_id, graphic_index),
    }
}
