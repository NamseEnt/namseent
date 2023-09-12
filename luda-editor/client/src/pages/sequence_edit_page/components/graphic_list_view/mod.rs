mod graphic_list_item;
mod graphic_thumbnail;
mod header;

use crate::{
    color,
    pages::sequence_edit_page::{
        atom::{EDITING_GRAPHIC_INDEX_ATOM, SEQUENCE_ATOM},
        components::graphic_list_view::{graphic_list_item::GraphicListItem, header::Header},
    },
};
use namui::prelude::*;
use namui_prebuilt::{scroll_view, simple_rect, table};

static DRAGGING: Atom<Option<DraggingContext>> = Atom::uninitialized_new();

#[component]
pub struct GraphicListView<'a> {
    pub project_id: Uuid,
    pub wh: Wh<Px>,
    pub selected_cut: Option<&'a rpc::data::Cut>,
}

impl Component for GraphicListView<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        const HEADER_HEIGHT: Px = px(32.0);
        const GRAPHIC_LIST_ITEM_HEIGHT: Px = px(48.0);
        const PADDING: Px = px(4.0);

        let Self {
            project_id,
            wh,
            selected_cut,
        } = self;
        let graphics = selected_cut.map(|cut| &cut.screen_graphics);
        let cut_id = selected_cut.map(|cut| cut.id);

        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());
        let (dragging, set_dragging) = ctx.atom_init(&DRAGGING, || None);
        let (editing_graphic_index, set_editing_graphic_index) =
            ctx.atom_init(&EDITING_GRAPHIC_INDEX_ATOM, || None);

        let on_mouse_move = |event: MouseEvent| {
            let Some(graphics) = graphics else {
                return;
            };
            if dragging.is_none() {
                return;
            }
            let cursor_located_graphic_index = {
                let local_y_in_content = event.local_xy().y + *scroll_y;
                ((local_y_in_content / GRAPHIC_LIST_ITEM_HEIGHT).round() as usize)
                    .clamp(0, graphics.len() - 1)
            };
            set_dragging.mutate(move |dragging| {
                let Some(dragging) = dragging else {
                    return;
                };
                dragging.end_index = cursor_located_graphic_index;
            })
        };
        let on_mouse_up = |_event: MouseEvent| {
            match (graphics, cut_id, *dragging) {
                (Some(graphics), Some(cut_id), Some(dragging))
                    if dragging.start_index != dragging.end_index =>
                {
                    let after_graphic_index = {
                        match dragging.end_index {
                            0 => None,
                            index => graphics.get(index).map(|(graphic_index, _)| *graphic_index),
                        }
                    };
                    SEQUENCE_ATOM.mutate(move |sequence| {
                        sequence.update_cut(
                            cut_id,
                            rpc::data::CutUpdateAction::ChangeGraphicOrder {
                                graphic_index: dragging.graphic_index,
                                after_graphic_index,
                            },
                        );
                    });
                }
                _ => {}
            };
            set_dragging.set(None);
        };

        let header_cell = table::hooks::fixed(HEADER_HEIGHT, |wh, ctx| {
            ctx.add(Header { wh });
        });

        let content = |ctx: &mut ComposeCtx| {
            let Some(graphics) = graphics else {
                return;
            };

            table::hooks::vertical(graphics.iter().map(|(graphic_index, graphic)| {
                table::hooks::fixed(GRAPHIC_LIST_ITEM_HEIGHT, |wh, ctx| {
                    table::hooks::padding(PADDING, |wh, ctx| {
                        let graphic_index = *graphic_index;
                        let is_selected = editing_graphic_index.as_ref() == &Some(graphic_index);
                        let graphic_list_item = GraphicListItem {
                            project_id,
                            wh,
                            graphic,
                            is_selected,
                        };
                        match *dragging {
                            Some(dragging) if dragging.graphic_index == graphic_index => ctx
                                .on_top()
                                .absolute(mouse::position() - dragging.thumbnail_clicked_offset_xy)
                                .add_with_key(
                                    graphic_index,
                                    graphic_list_item.with_mouse_cursor(MouseCursor::Grabbing),
                                ),
                            _ => ctx.add_with_key(
                                graphic_index,
                                graphic_list_item
                                    .attach_event(|event| {
                                        if let namui::Event::MouseDown { event } = event {
                                            if event.is_local_xy_in()
                                                && (event.button == Some(MouseButton::Left))
                                            {
                                                let start_index = graphics
                                                    .iter()
                                                    .position(|(index, _)| index == &graphic_index)
                                                    .unwrap();
                                                event.stop_propagation();
                                                set_editing_graphic_index.set(Some(graphic_index));
                                                set_dragging.set(Some(DraggingContext {
                                                    graphic_index,
                                                    thumbnail_clicked_offset_xy: event.local_xy(),
                                                    start_index,
                                                    end_index: start_index,
                                                }));
                                            }
                                        }
                                    })
                                    .with_mouse_cursor(MouseCursor::Pointer),
                            ),
                        };
                    })(wh, ctx);
                })
            }))(wh, ctx);
        };

        let body_cell = table::hooks::ratio(1, |wh, ctx| {
            ctx.compose(|ctx| {
                const SIDE_MARGIN: Px = px(24.0);
                const STROKE_WIDTH: Px = px(4.0);
                let Some(dragging) = *dragging else {
                    return;
                };
                let cursor_y =
                    GRAPHIC_LIST_ITEM_HEIGHT * ((dragging.end_index) as f32) - *scroll_y + PADDING;
                let path = Path::new()
                    .move_to(SIDE_MARGIN, cursor_y)
                    .line_to(wh.width - SIDE_MARGIN, cursor_y);
                let paint = Paint::new()
                    .set_style(PaintStyle::Stroke)
                    .set_stroke_width(STROKE_WIDTH)
                    .set_stroke_cap(StrokeCap::Round)
                    .set_color(color::STROKE_FOCUS);
                ctx.add(namui::path(path, paint));
            });

            table::hooks::padding(PADDING, |wh, ctx| {
                ctx.add(scroll_view::ScrollViewWithCtx {
                    xy: Xy::zero(),
                    scroll_bar_width: 4.px(),
                    height: wh.height,
                    content,
                    scroll_y: *scroll_y,
                    set_scroll_y,
                });
            })(wh, ctx);

            ctx.attach_event(|event| match event {
                namui::Event::MouseDown { event } => {
                    namui::log!("MouseDown: {:?}", event.local_xy().y + *scroll_y);
                }
                namui::Event::MouseMove { event } => {
                    on_mouse_move(event);
                }
                namui::Event::MouseUp { event } => {
                    on_mouse_up(event);
                }
                _ => {}
            });
        });

        ctx.compose(|ctx| {
            table::hooks::vertical([header_cell, body_cell])(wh, ctx);
        });

        ctx.component(simple_rect(
            wh,
            color::STROKE_NORMAL,
            1.px(),
            color::BACKGROUND,
        ));

        ctx.done()
    }
}

#[derive(Debug, Clone, Copy)]
struct DraggingContext {
    graphic_index: Uuid,
    thumbnail_clicked_offset_xy: Xy<Px>,
    start_index: usize,
    end_index: usize,
}
