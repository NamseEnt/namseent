use super::{render_in_game_menu, render_start_menu};
use namui::{on_top, screen, Code, RenderingTree};
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
        event.is::<InternalEvent>(|event| match event {
            InternalEvent::CloseRequested => self.close(),
            InternalEvent::ChangeTabRequested(tab) => self.tab = tab.clone(),
            InternalEvent::EscapeKeyDown => {
                self.toggle_ingame_menu();
            }
        });
    }

    pub fn render(&self) -> RenderingTree {
        let wh = screen::size();
        let tree = match self.open {
            true => on_top(event_trap(match self.tab {
                Tab::Start => render_start_menu(wh),
                Tab::InGame => render_in_game_menu(wh),
            })),
            false => RenderingTree::Empty,
        };

        tree.attach_event(|builder| {
            builder.on_key_down(|event| {
                if event.code == Code::Escape {
                    namui::event::send(InternalEvent::EscapeKeyDown);
                }
            });
        })
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
    EscapeKeyDown,
}
