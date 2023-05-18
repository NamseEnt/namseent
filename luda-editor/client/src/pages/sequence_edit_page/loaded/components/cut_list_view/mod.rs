mod render;
mod update;

use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::{Cut, Memo};
use std::collections::HashMap;

pub struct CutListView {
    list_view: list_view::ListView,
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub cuts: &'a Vec<Cut>,
    pub selected_cut_id: Option<Uuid>,
    pub is_focused: bool,
    pub cut_id_memo_map: &'a HashMap<Uuid, Vec<Memo>>,
}

pub enum Event {
    RightClick { global_xy: Xy<Px> },
    ClickCut { cut_id: Uuid },
    MoveToNextCutByKeyboard { next_cut_id: Uuid },
    PressEnterOnCut { cut_id: Uuid },
}

impl CutListView {
    pub fn new() -> Self {
        Self {
            list_view: list_view::ListView::new(),
        }
    }
}
