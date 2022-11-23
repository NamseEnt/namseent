use namui::prelude::*;
use namui_prebuilt::*;

#[derive(Debug, Clone)]
pub struct ContextMenu {
    global_xy: Xy<Px>,
    list_view: list_view::ListView,
    sequence_id: namui::Uuid,
}
pub enum Event {
    DeleteButtonClicked { sequence_id: namui::Uuid },
    RenameButtonClicked { sequence_id: namui::Uuid },
}
impl ContextMenu {
    pub fn new(global_xy: Xy<Px>, sequence_id: namui::Uuid) -> Self {
        Self {
            global_xy,
            list_view: list_view::ListView::new(),
            sequence_id,
        }
    }
    pub fn update(&mut self, event: &namui::Event) {
        self.list_view.update(event);
    }
    pub fn render(&self) -> namui::RenderingTree {
        let context_menu_item_wh = Wh::new(100.px(), 40.px());
        #[derive(Clone, Copy)]
        enum ContextMenuItem {
            Delete,
            Rename,
        }
        let context_menu_items = [ContextMenuItem::Delete, ContextMenuItem::Rename];

        absolute(
            self.global_xy.x,
            self.global_xy.y,
            self.list_view.render(list_view::Props {
                xy: Xy::single(0.px()),
                height: context_menu_item_wh.height * context_menu_items.len(),
                scroll_bar_width: 0.px(),
                item_wh: context_menu_item_wh,
                items: context_menu_items,
                item_render: |wh, item| {
                    let text = match item {
                        ContextMenuItem::Delete => "  Delete",
                        ContextMenuItem::Rename => "  Rename",
                    };
                    render([
                        simple_rect(wh, Color::WHITE, 1.px(), Color::grayscale_f01(0.5)),
                        namui_prebuilt::typography::body::left(wh.height, text, Color::WHITE),
                    ])
                    .attach_event(|builder| {
                        let sequence_id = self.sequence_id;
                        builder.on_mouse_up_in(move |_event| match item {
                            ContextMenuItem::Delete => {
                                namui::event::send(Event::DeleteButtonClicked { sequence_id });
                            }
                            ContextMenuItem::Rename => {
                                namui::event::send(Event::RenameButtonClicked { sequence_id });
                            }
                        });
                    })
                },
            }),
        )
    }
}
