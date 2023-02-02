use super::{render_in_game_menu, render_start_menu};
use namui::{on_top, screen, RenderingTree};
use namui_prebuilt::event_trap;

pub struct Menu {
    pub open: bool,
    pub tab: Tab,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            open: true,
            tab: Tab::Start,
        }
    }

    pub fn render(&self) -> RenderingTree {
        match self.open {
            true => {
                let wh = screen::size();
                on_top(event_trap(match self.tab {
                    Tab::Start => render_start_menu(wh),
                    Tab::InGame => render_in_game_menu(wh),
                }))
            }
            false => RenderingTree::Empty,
        }
    }
}

pub enum Tab {
    Start,
    InGame,
}
