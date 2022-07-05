use super::*;
use namui::prelude::*;
use namui_prebuilt::{
    button::text_button,
    table::{fixed, horizontal, ratio, vertical},
};

pub struct AnimationEditor {
    graph_editor: graph_editor::GraphEditor,
    time_point_editor: time_point_editor::TimePointEditor,
    animation_history: AnimationHistory,
    selected_tab: Tab,
    layer_list_window: layer_list_window::LayerListWindow,
}

pub struct Props {
    pub wh: Wh<f32>,
}

enum Event {
    TabButtonMouseDown { tab: Tab },
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tab {
    TimePointEditor,
    GraphEditor,
}

impl AnimationEditor {
    pub fn new(animation: &animation::Animation) -> Self {
        let animation_history = AnimationHistory::new(animation.clone());
        Self {
            graph_editor: graph_editor::GraphEditor::new(animation_history.clone()),
            time_point_editor: time_point_editor::TimePointEditor::new(animation_history.clone()),
            layer_list_window: layer_list_window::LayerListWindow::new(animation_history.clone()),
            animation_history,
            selected_tab: Tab::TimePointEditor,
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::TabButtonMouseDown { tab } => {
                    self.selected_tab = *tab;
                }
            }
        }

        match self.selected_tab {
            Tab::TimePointEditor => self.time_point_editor.update(event),
            Tab::GraphEditor => self.graph_editor.update(event),
        }

        self.layer_list_window.update(event);
    }
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let animation = self.animation_history.get_preview();

        let render_tab_button = |tab: Tab, wh: Wh<f32>| {
            let is_selected_tab = self.selected_tab == tab;
            text_button(
                XywhRect::from_xy_wh(Xy { x: 0.0, y: 0.0 }, wh),
                match tab {
                    Tab::TimePointEditor => "Timeline",
                    Tab::GraphEditor => "Graph",
                },
                match is_selected_tab {
                    true => Color::WHITE,
                    false => Color::BLACK,
                },
                match is_selected_tab {
                    true => Color::WHITE,
                    false => Color::BLACK,
                },
                1.0,
                match is_selected_tab {
                    true => Color::BLACK,
                    false => Color::WHITE,
                },
                move || {
                    namui::event::send(Event::TabButtonMouseDown { tab });
                },
            )
        };

        vertical([
            fixed(
                40.0,
                horizontal([
                    ratio(1.0, |wh| render_tab_button(Tab::TimePointEditor, wh)),
                    ratio(1.0, |wh| render_tab_button(Tab::GraphEditor, wh)),
                ]),
            ),
            ratio(1.0, |wh| match self.selected_tab {
                Tab::TimePointEditor => self.time_point_editor.render(time_point_editor::Props {
                    wh,
                    animation: &animation,
                    layer_list_window: &self.layer_list_window,
                }),
                Tab::GraphEditor => self.graph_editor.render(graph_editor::Props {
                    wh,
                    animation: &animation,
                    layer_list_window: &self.layer_list_window,
                }),
            }),
        ])(props.wh)
    }
}
