mod render;
mod update;

use namui::prelude::*;
use namui_prebuilt::scroll_view;
use rpc::data::Memo;

pub struct MemoListView {
    scroll_view: scroll_view::ScrollView,
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub memos: &'a Vec<Memo>,
    pub sequence_id: Uuid,
    pub user_id: Uuid,
}

impl MemoListView {
    pub fn new() -> Self {
        Self {
            scroll_view: scroll_view::ScrollView::new(),
        }
    }
}

pub enum Event {
    MemoDeleted { cut_id: Uuid, memo_id: Uuid },
}
