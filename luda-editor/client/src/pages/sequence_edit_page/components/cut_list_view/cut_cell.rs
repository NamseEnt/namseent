use super::*;
use crate::{
    components::{
        cg_render::CgRender,
        context_menu::{if_context_menu_for, open_context_menu},
        sequence_player::get_inner_content_rect,
    },
    pages::sequence_edit_page::atom::SEQUENCE_ATOM,
    storage::{get_project_cg_thumbnail_image_url, get_project_image_url},
    *,
};
use namui_prebuilt::table::*;
use rpc::data::{CgFile, ScreenGraphic, SequenceUpdateAction};

#[namui::component]
pub struct CutCell<'a> {
    pub wh: Wh<Px>,
    pub index: usize,
    pub cut: Cut,
    pub memo_count: usize,
    pub is_selected: bool,
    pub is_focused: bool,
    pub on_click: callback!('a, Uuid),
    pub project_id: Uuid,
    pub cg_files: &'a Vec<CgFile>,
}

enum ContextMenu {
    CutCell { cut_id: Uuid },
}

impl Component for CutCell<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self {
            wh,
            index,
            cut,
            memo_count,
            is_selected,
            is_focused,
            on_click,
            project_id,
            cg_files,
        } = self;

        let stroke_color = color::stroke_color(is_selected, is_focused);
        let cut_id = cut.id;
        let (dragging, set_dragging) = ctx.atom(&DRAGGING_CONTEXT);

        if_context_menu_for::<ContextMenu>(|context_menu, builder| match context_menu {
            &ContextMenu::CutCell { cut_id } => builder.add_button("Delete Cut", || {
                SEQUENCE_ATOM.mutate(move |sequence| {
                    sequence.update(SequenceUpdateAction::DeleteCut { cut_id })
                });
            }),
        });

        ctx.compose(|ctx| {
            let Some(dragging) = *dragging else {
                return;
            };
            if dragging.cut_id != cut_id {
                return;
            }
            ctx.add(
                simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::from_u8(0, 0, 0, 128))
                    .attach_event(|event| {
                        if let namui::Event::MouseDown { event } = event {
                            if event.is_local_xy_in() {
                                event.stop_propagation();
                            }
                        }
                    })
                    .with_mouse_cursor(MouseCursor::Grabbing),
            );
        });

        ctx.component(transparent_rect(wh).attach_event(|event| {
            if let namui::Event::MouseDown { event } = event {
                if event.is_local_xy_in() && event.button == Some(MouseButton::Left) {
                    on_click(cut_id);
                }
            }
        }));

        ctx.compose(|ctx| {
            table::padding(
                12.px(),
                table::horizontal([
                    table::fixed(24.px(), |wh, ctx| {
                        table::vertical([
                            table::fit(
                                table::FitAlign::LeftTop,
                                typography::body::center_top(
                                    wh.width,
                                    format!("{}", index),
                                    stroke_color,
                                ),
                            ),
                            table::fixed(4.px(), |_, _| {}),
                            table::fit(
                                table::FitAlign::LeftTop,
                                render_comment_badge(wh.width, memo_count, stroke_color),
                            ),
                        ])(wh, ctx)
                    }),
                    table::ratio(1, |wh, ctx| {
                        let thumbnail = Thumbnail {
                            wh,
                            cut: &cut,
                            is_focused,
                            is_selected,
                            project_id,
                            cg_files,
                        };
                        match *dragging {
                            Some(dragging) if dragging.cut_id == cut_id => {
                                ctx.on_top()
                                    .absolute(
                                        mouse::position() - dragging.thumbnail_clicked_offset_xy,
                                    )
                                    .add(thumbnail);
                            }
                            _ => {
                                ctx.add(thumbnail).attach_event(|event| {
                                    if let namui::Event::MouseDown { event } = event {
                                        if event.is_local_xy_in() {
                                            if event.button == Some(MouseButton::Left) {
                                                set_dragging.set(Some(DraggingContext {
                                                    cut_id,
                                                    thumbnail_clicked_offset_xy: event.local_xy(),
                                                    end_index: index,
                                                }));
                                            }
                                            if event.button == Some(MouseButton::Right) {
                                                open_context_menu(
                                                    event.global_xy,
                                                    ContextMenu::CutCell { cut_id },
                                                );
                                                event.stop_propagation();
                                            }
                                        }
                                    }
                                });
                            }
                        };
                    }),
                    table::fixed(8.px(), |_wh, _ctx| {}),
                ]),
            )(wh, ctx)
        });

        
    }
}

fn render_comment_badge(width: Px, memo_count: usize, color: Color) -> RenderingTree {
    if memo_count == 0 {
        return RenderingTree::Empty;
    }

    let memo_count = if memo_count > 9 {
        "9+".to_string()
    } else {
        memo_count.to_string()
    };

    let path_builder = Path::new()
        .move_to(0.05.px(), 0.05.px())
        .line_to(0.95.px(), 0.05.px())
        .line_to(0.95.px(), 0.7.px())
        .line_to(0.8.px(), 0.7.px())
        .line_to(0.9.px(), 0.8.px())
        .line_to(0.6.px(), 0.7.px())
        .line_to(0.05.px(), 0.7.px())
        .line_to(0.05.px(), 0.05.px())
        .scale(width.as_f32(), width.as_f32());

    let paint = Paint::new().set_style(PaintStyle::Fill).set_color(color);

    render([
        text(TextParam {
            text: memo_count,
            x: width * 0.5,
            y: width * 0.35,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font: Font {
                size: (width * 0.5).into(),
                name: "NotoSansKR-Bold".to_string(),
            },
            style: TextStyle {
                border: None,
                drop_shadow: None,
                color: color::BACKGROUND,
                background: None,
                line_height_percent: 100.percent(),
                underline: None,
            },
            max_width: width.into(),
        }),
        path(path_builder, paint),
    ])
}

struct Thumbnail<'a> {
    wh: Wh<Px>,
    cut: &'a Cut,
    is_selected: bool,
    is_focused: bool,
    project_id: Uuid,
    cg_files: &'a Vec<CgFile>,
}
impl Component for Thumbnail<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self {
            wh,
            cut,
            is_selected,
            is_focused,
            project_id,
            cg_files,
        } = self;

        let stroke_color = color::stroke_color(is_selected, is_focused);
        let screen_graphics = &cut.screen_graphics;
        let inner_content_rect = get_inner_content_rect(self.wh);

        ctx.component(simple_rect(
            wh,
            stroke_color,
            if is_selected { 2.px() } else { 1.px() },
            Color::TRANSPARENT,
        ));

        ctx.compose(|ctx| {
            let mut ctx = ctx
                .clip(Path::new().add_rect(inner_content_rect), ClipOp::Intersect)
                .translate(inner_content_rect.xy());

            ctx.add(TextBox {
                container_wh: inner_content_rect.wh(),
                cut,
            });

            for (graphic_index, screen_graphic) in screen_graphics.iter() {
                ctx.add_with_key(
                    graphic_index,
                    GraphicClip {
                        container_wh: inner_content_rect.wh(),
                        project_id,
                        screen_graphic,
                        cg_files,
                    },
                );
            }
        });

        
    }
}

struct GraphicClip<'a> {
    container_wh: Wh<Px>,
    project_id: Uuid,
    screen_graphic: &'a ScreenGraphic,
    cg_files: &'a Vec<CgFile>,
}
impl Component for GraphicClip<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self {
            container_wh,
            project_id,
            screen_graphic,
            cg_files,
        } = self;

        let url = match &screen_graphic {
            ScreenGraphic::Image(image) => get_project_image_url(project_id, image.id).unwrap(),
            ScreenGraphic::Cg(cg) => get_project_cg_thumbnail_image_url(project_id, cg.id).unwrap(),
        };
        let image = ctx.image(&url);
        let image = match image.as_ref() {
            Some(Ok(image)) => image,
            Some(Err(error)) => {
                namui::log!("Failed to load image: {:?}", error);
                return ;
            }
            None => {
                return ;
            }
        };
        let graphic_wh = image.wh;
        let circumscribed = screen_graphic.circumscribed();

        let screen_radius = container_wh.length() / 2;
        let graphic_radius_px = graphic_wh.length() / 2;
        let radius_px = screen_radius * circumscribed.radius;
        let graphic_wh_on_screen = graphic_wh * (radius_px / graphic_radius_px);

        let center_xy = container_wh.as_xy() * circumscribed.center_xy;

        let graphic_rendering_rect = {
            let image_left_top_xy = center_xy - graphic_wh_on_screen.as_xy() / 2.0;
            Rect::from_xy_wh(image_left_top_xy, graphic_wh_on_screen)
        };

        let graphic_rendering_tree = |ctx: &mut ComposeCtx| match &screen_graphic {
            ScreenGraphic::Image(_image) => {
                ctx.add(namui::image(ImageParam {
                    rect: graphic_rendering_rect,
                    source: image.src.clone(),
                    style: ImageStyle {
                        fit: ImageFit::Fill,
                        paint: None,
                    },
                }));
            }
            ScreenGraphic::Cg(cg) => {
                let Some(cg_file) = cg_files.iter().find(|cg_file| cg_file.name == cg.name) else {
                    return;
                };
                ctx.add(CgRender {
                    project_id,
                    rect: graphic_rendering_rect,
                    screen_cg: cg,
                    cg_file,
                });
            }
        };

        ctx.compose(|ctx| {
            let center_xy = graphic_rendering_rect.center();
            let mut ctx = ctx
                .translate(center_xy)
                .rotate(screen_graphic.rotation())
                .translate(center_xy * -1.0);

            ctx.compose(graphic_rendering_tree);
        });

        
    }
}

struct TextBox<'a> {
    container_wh: Wh<Px>,
    cut: &'a Cut,
}
impl Component for TextBox<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self { container_wh, cut } = self;

        const PADDING_RATIO: f32 = 32.0 / 1080.0;
        const TEXT_BOARDER_RATIO: f32 = 4.0 / 1080.0;
        const CHARACTER_NAME_SIZE_RATIO: f32 = 36.0 / 1080.0;
        const LINE_SIZE_RATIO: f32 = 24.0 / 1080.0;

        let padding = container_wh.height * PADDING_RATIO;
        let text_boarder = container_wh.height * TEXT_BOARDER_RATIO;
        let character_name_size = container_wh.height * CHARACTER_NAME_SIZE_RATIO;
        let line_size = container_wh.height * LINE_SIZE_RATIO;

        let character_name_font = Font {
            size: character_name_size.into(),
            name: "NotoSansKR-Bold".to_string(),
        };
        let cut_text_font = Font {
            size: line_size.into(),
            ..character_name_font.clone()
        };

        let character_name_text_style = TextStyle {
            border: Some(TextStyleBorder {
                width: text_boarder,
                color: Color::BLACK,
            }),
            drop_shadow: Some(TextStyleDropShadow {
                x: 1.px(),
                y: 2.px(),
                color: Some(Color::BLACK),
            }),
            color: Color::WHITE,
            ..Default::default()
        };
        let cut_text_style = TextStyle {
            line_height_percent: 150.percent(),
            ..character_name_text_style.clone()
        };

        let character_name_side = |wh: Wh<Px>, ctx: &mut ComposeCtx| {
            ctx.add(text(TextParam {
                text: cut.character_name.clone(),
                x: 0.px(),
                y: wh.height / 2,
                align: TextAlign::Left,
                baseline: TextBaseline::Middle,
                font: character_name_font,
                style: character_name_text_style,
                max_width: Some(wh.width),
            }));
        };

        let cut_text_side = |wh: Wh<Px>, ctx: &mut ComposeCtx| {
            ctx.add(text(TextParam {
                text: cut.line.clone(),
                x: 0.px(),
                y: 0.px(),
                align: TextAlign::Left,
                baseline: TextBaseline::Top,
                font: cut_text_font,
                style: cut_text_style,
                max_width: Some(wh.width),
            }));
        };

        ctx.compose(|ctx| {
            vertical([
                ratio(3, |_wh, _ctx| {}),
                ratio(1, |wh, ctx| {
                    vertical([
                        ratio(1, horizontal_padding(padding, character_name_side)),
                        ratio(3, padding_no_clip(padding, cut_text_side)),
                    ])(wh, ctx);

                    ctx.add(simple_rect(
                        wh,
                        Color::TRANSPARENT,
                        0.px(),
                        Color::from_f01(1.0, 1.0, 1.0, 0.3),
                    ));
                }),
            ])(container_wh, ctx)
        });

        
    }
}
