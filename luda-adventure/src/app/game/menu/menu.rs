use super::{render_in_game_menu, render_start_menu};
use namui::{on_top, screen, Code, NamuiEvent, RenderingTree};
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

    pub fn update(&mut self, event: &namui::Event) {
        event
            .is::<NamuiEvent>(|event| match event {
                NamuiEvent::KeyDown(event) => {
                    if event.code == Code::Escape {
                        self.toggle_ingame_menu();
                    }
                }
                _ => (),
            })
            .is::<InternalEvent>(|event| match event {
                InternalEvent::CloseRequested => self.close(),
                InternalEvent::ChangeTabRequested(tab) => self.tab = tab.clone(),
            });
    }

    pub fn render(&self) -> RenderingTree {
        if !self.open {
            return RenderingTree::Empty;
        }

        let wh = screen::size();
        on_top(event_trap(match self.tab {
            Tab::Start => render_start_menu(wh),
            Tab::InGame => render_in_game_menu(wh),
        }))
    }

    fn toggle_ingame_menu(&mut self) {
        if let Tab::InGame = self.tab {
            self.open = !self.open;
        }
    }

    fn close(&mut self) {
        self.open = false;
    }

    pub fn request_close() {
        namui::event::send(InternalEvent::CloseRequested);
    }

    pub fn request_change_tab(tab: Tab) {
        namui::event::send(InternalEvent::ChangeTabRequested(tab));
    }
}

#[derive(Clone)]
pub enum Tab {
    Start,
    InGame,
}

enum InternalEvent {
    CloseRequested,
    ChangeTabRequested(Tab),
}
