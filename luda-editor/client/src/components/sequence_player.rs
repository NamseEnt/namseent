use super::cg_render;
use crate::storage::{get_project_cg_thumbnail_image_url, get_project_image_url};
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::*;

pub struct SequencePlayer {
    sequence: Sequence,
    project_shared_data: ProjectSharedData,
    state: State,
    cg_files: Option<Vec<CgFile>>,
}

pub struct Props {
    pub wh: Wh<Px>,
}

enum InternalEvent {
    OnClickScreen,
    GoToPrevCut,
    GoToNextCut,
    CgFilesLoaded(Vec<CgFile>),
}

enum State {
    ShowingCut {
        cut_index: usize,
    },
    Transitioning {
        from_cut_index: usize,
        transition_progress: OneZero,
        start_time: Time,
    },
}

// impl SequencePlayer {
//     pub fn new(
//         sequence: Sequence,
//         project_shared_data: ProjectSharedData,
//         start_cut_index: usize,
//         cg_files: Option<Vec<CgFile>>,
//     ) -> Self {
//         let project_id = project_shared_data.id();
//         if cg_files.is_none() {
//             spawn_local(async move {
//                 let response = crate::RPC
//                     .list_cg_files(rpc::list_cg_files::Request { project_id })
//                     .await
//                     .unwrap();
//                 todo!()
//                 // namui::event::send(InternalEvent::CgFilesLoaded(response.cg_files));
//             });
//         }
//         Self {
//             sequence,
//             project_shared_data,
//             state: State::ShowingCut {
//                 cut_index: start_cut_index,
//             },
//             cg_files,
//         }
//     }
//     pub fn render(&self, props: Props) -> RenderingTree {
//         if self.sequence.cuts.is_empty() {
//             return RenderingTree::Empty;
//         }
//         let Some(cg_files) = &self.cg_files else {
//             return RenderingTree::Empty;
//         };

//         let inner_content_rect = get_inner_content_rect(props.wh);

//         translate(
//             inner_content_rect.x(),
//             inner_content_rect.y(),
//             match &self.state {
//                 &State::ShowingCut { cut_index } => {
//                     let cut = self.sequence.cuts.get(cut_index).unwrap();
//                     render([
//                         render_graphics(
//                             self.project_shared_data.id(),
//                             inner_content_rect.wh(),
//                             cut,
//                             1.0.one_zero(),
//                             &cg_files,
//                         ),
//                         render_text_box(inner_content_rect.wh()),
//                         render_text(
//                             &self.project_shared_data,
//                             inner_content_rect.wh(),
//                             cut,
//                             1.0.one_zero(),
//                         ),
//                         simple_rect(
//                             inner_content_rect.wh(),
//                             Color::TRANSPARENT,
//                             0.px(),
//                             Color::TRANSPARENT,
//                         )
//                         .attach_event(|builder| {
//                             builder.on_mouse_down_in(|_| {
//                                 todo!()
//                                 // namui::event::send(InternalEvent::OnClickScreen);
//                             });
//                         }),
//                     ])
//                 }
//                 &State::Transitioning {
//                     from_cut_index,
//                     transition_progress,
//                     start_time: _start_time,
//                 } => {
//                     let from_cut = self.sequence.cuts.get(from_cut_index).unwrap();
//                     render([
//                         self.render_transitioning_image(
//                             inner_content_rect.wh(),
//                             from_cut_index,
//                             transition_progress,
//                             &cg_files,
//                         ),
//                         render_text_box(inner_content_rect.wh()),
//                         render_text(
//                             &self.project_shared_data,
//                             inner_content_rect.wh(),
//                             from_cut,
//                             1.0.one_zero() - transition_progress,
//                         ),
//                     ])
//                 }
//             },
//         )
//         .attach_event(|builder| {
//             builder.on_key_down(|event: KeyboardEvent| {
//                 if event.code == Code::ArrowLeft {
//                     todo!()
//                     // namui::event::send(InternalEvent::GoToPrevCut);
//                 } else if event.code == Code::ArrowRight {
//                     todo!()
//                     // namui::event::send(InternalEvent::GoToNextCut);
//                 }
//             });
//         })
//     }
//     // pub fn update(&mut self, event: &namui::Event) {
//     //     match &mut self.state {
//     //         State::ShowingCut { .. } => {}
//     //         State::Transitioning {
//     //             from_cut_index,
//     //             transition_progress,
//     //             start_time,
//     //         } => {
//     //             let transition_duration = 500.ms();

//     //             let delta_time = namui::now() - *start_time;
//     //             if delta_time > transition_duration {
//     //                 self.state = State::ShowingCut {
//     //                     cut_index: *from_cut_index + 1,
//     //                 };
//     //             } else {
//     //                 *transition_progress = (delta_time / transition_duration).one_zero();
//     //             }
//     //         }
//     //     }

//     //     event.is::<InternalEvent>(|event| match event {
//     //         InternalEvent::OnClickScreen => {
//     //             self.go_to_next_cut(true);
//     //         }
//     //         InternalEvent::GoToPrevCut => self.go_to_prev_cut(),
//     //         InternalEvent::GoToNextCut => self.go_to_next_cut(false),
//     //         InternalEvent::CgFilesLoaded(cg_files) => {
//     //             self.cg_files = Some(cg_files.clone());
//     //         }
//     //     });
//     // }

//     fn go_to_next_cut(&mut self, do_transition: bool) {
//         if self.sequence.cuts.len() == 0 {
//             return;
//         }

//         match self.state {
//             State::ShowingCut { cut_index } => {
//                 let next_cut_index = cut_index + 1;
//                 if next_cut_index >= self.sequence.cuts.len() {
//                     return;
//                 }

//                 if do_transition {
//                     self.state = State::Transitioning {
//                         from_cut_index: cut_index,
//                         transition_progress: 0.0.one_zero(),
//                         start_time: namui::now(),
//                     };
//                 } else {
//                     self.state = State::ShowingCut {
//                         cut_index: next_cut_index,
//                     };
//                 }
//             }
//             State::Transitioning { from_cut_index, .. } => {
//                 if !do_transition {
//                     self.state = State::ShowingCut {
//                         cut_index: from_cut_index + 1,
//                     };
//                 }
//             }
//         }
//     }

//     fn go_to_prev_cut(&mut self) {
//         let prev_cut_index = match self.state {
//             State::ShowingCut { cut_index } => {
//                 if cut_index == 0 {
//                     return;
//                 }
//                 cut_index - 1
//             }
//             State::Transitioning { from_cut_index, .. } => from_cut_index,
//         };

//         self.state = State::ShowingCut {
//             cut_index: prev_cut_index,
//         };
//     }

//     fn render_transitioning_image(
//         &self,
//         wh: Wh<Px>,
//         from_cut_index: usize,
//         transition_progress: OneZero,
//         cg_files: &Vec<CgFile>,
//     ) -> RenderingTree {
//         let from_cut = self.sequence.cuts.get(from_cut_index).unwrap();
//         let to_cut = self.sequence.cuts.get(from_cut_index + 1).unwrap();

//         let project_id = self.project_shared_data.id();

//         if from_cut.screen_graphics == to_cut.screen_graphics {
//             render_graphics(project_id, wh, from_cut, 1.0.one_zero(), cg_files)
//         } else {
//             let from_opacity = 1.0.one_zero() - transition_progress;
//             let to_opacity = transition_progress;

//             render([
//                 render_graphics(project_id, wh, from_cut, from_opacity, cg_files),
//                 render_graphics(project_id, wh, to_cut, to_opacity, cg_files),
//             ])
//         }
//     }
// }

pub fn get_inner_content_rect(wh: Wh<Px>) -> Rect<Px> {
    let width_per_height = 4.0 / 3.0;

    let ratio = wh.width / wh.height;

    if ratio == width_per_height {
        Rect::from_xy_wh(Xy::zero(), wh)
    } else if ratio > width_per_height {
        let result_wh = Wh::new(wh.height * width_per_height, wh.height);
        let result_xy = Xy::new((wh.width - result_wh.width) / 2.0, 0.px());
        Rect::from_xy_wh(result_xy, result_wh)
    } else {
        let result_wh = Wh::new(wh.width, wh.width / width_per_height);
        let result_xy = Xy::new(0.px(), (wh.height - result_wh.height) / 2.0);
        Rect::from_xy_wh(result_xy, result_wh)
    }
}

pub fn render_text_box(screen_wh: Wh<Px>) -> RenderingTree {
    table::vertical([
        table::ratio(3, |_wh| RenderingTree::Empty),
        table::ratio(1, |wh| {
            rect(RectParam {
                rect: Rect::from_xy_wh(Xy::zero(), wh),
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: Color::BLACK,
                        width: 1.px(),
                        border_position: BorderPosition::Inside,
                    }),
                    fill: Some(RectFill {
                        color: Color::from_f01(1.0, 1.0, 1.0, 0.3),
                    }),
                    round: Some(RectRound { radius: 8.px() }),
                },
            })
        }),
    ])(screen_wh)
}

pub const CHARACTER_NAME_FONT: FontType = FontType {
    serif: false,
    size: int_px(36),
    language: Language::Ko,
    font_weight: FontWeight::BOLD,
};
pub fn character_name_text_style(opacity: OneZero) -> TextStyle {
    TextStyle {
        border: Some(TextStyleBorder {
            width: 4.px(),
            color: Color::from_f01(0.0, 0.0, 0.0, opacity.as_f32()),
        }),
        drop_shadow: Some(TextStyleDropShadow {
            x: 1.px(),
            y: 2.px(),
            color: Some(Color::from_f01(0.0, 0.0, 0.0, opacity.as_f32())),
        }),
        color: Color::from_f01(1.0, 1.0, 1.0, opacity.as_f32()),
        ..Default::default()
    }
}

pub const CUT_TEXT_FONT: FontType = FontType {
    serif: false,
    size: int_px(24),
    language: Language::Ko,
    font_weight: FontWeight::BOLD,
};
pub fn cut_text_style(opacity: OneZero) -> TextStyle {
    TextStyle {
        border: Some(TextStyleBorder {
            width: 4.px(),
            color: Color::from_f01(0.0, 0.0, 0.0, opacity.as_f32()),
        }),
        drop_shadow: Some(TextStyleDropShadow {
            x: 1.px(),
            y: 2.px(),
            color: Some(Color::from_f01(0.0, 0.0, 0.0, opacity.as_f32())),
        }),
        color: Color::from_f01(1.0, 1.0, 1.0, opacity.as_f32()),
        line_height_percent: 150.percent(),
        ..Default::default()
    }
}

pub fn render_text(
    _project_shared_data: &ProjectSharedData,
    wh: Wh<Px>,
    cut: &Cut,
    opacity: OneZero,
) -> RenderingTree {
    render_over_text(
        wh,
        |wh| {
            text(TextParam {
                text: cut.character_name.clone(),
                x: 0.px(),
                y: wh.height / 2,
                align: TextAlign::Left,
                baseline: TextBaseline::Middle,
                font_type: CHARACTER_NAME_FONT,
                style: character_name_text_style(opacity),
                max_width: Some(wh.width),
            })
        },
        |wh| {
            text(TextParam {
                text: cut.line.clone(),
                x: 0.px(),
                y: 0.px(),
                align: TextAlign::Left,
                baseline: TextBaseline::Top,
                font_type: CUT_TEXT_FONT,
                style: cut_text_style(opacity),
                max_width: Some(wh.width),
            })
        },
    )
}

pub fn render_over_text(
    wh: Wh<Px>,
    character_name_side: impl FnOnce(Wh<Px>) -> RenderingTree,
    cut_text_side: impl FnOnce(Wh<Px>) -> RenderingTree,
) -> RenderingTree {
    table::vertical([
        table::ratio(3, |_wh| RenderingTree::Empty),
        table::ratio(
            1,
            table::vertical([
                table::ratio(1, table::horizontal_padding(32.px(), character_name_side)),
                table::ratio(3, table::padding_no_clip(32.px(), cut_text_side)),
            ]),
        ),
    ])(wh)
}

pub fn render_over_text_hooks<'a>(
    ctx: &mut ComposeCtx,
    wh: Wh<Px>,
    character_name_side: impl 'a + Fn(Wh<Px>, &mut ComposeCtx),
    cut_text_side: impl 'a + Fn(Wh<Px>, &mut ComposeCtx),
) {
    table::hooks::vertical([
        table::hooks::ratio(3, |_wh, _ctx| {}),
        table::hooks::ratio(
            1,
            table::hooks::vertical([
                table::hooks::ratio(
                    1,
                    table::hooks::horizontal_padding(32.px(), character_name_side),
                ),
                table::hooks::ratio(3, table::hooks::padding_no_clip(32.px(), cut_text_side)),
            ]),
        ),
    ])(wh, ctx);
}

pub fn calculate_graphic_wh_on_screen(
    original_graphic_size: Wh<Px>,
    container_wh: Wh<Px>,
    circumscribed: Circumscribed<Percent>,
) -> Wh<Px> {
    let screen_radius = container_wh.length() / 2;
    let image_radius_px = original_graphic_size.length() / 2;
    let radius_px = screen_radius * circumscribed.radius;

    let wh = original_graphic_size * (radius_px / image_radius_px);
    wh
}

pub fn calculate_graphic_rect_on_screen(
    original_graphic_size: Wh<Px>,
    container_wh: Wh<Px>,
    circumscribed: Circumscribed<Percent>,
) -> Rect<Px> {
    let wh = calculate_graphic_wh_on_screen(original_graphic_size, container_wh, circumscribed);
    let center_xy = container_wh.as_xy() * circumscribed.center_xy;

    let xy = center_xy - wh.as_xy() / 2.0;

    Rect::from_xy_wh(xy, wh)
}

pub fn render_graphics(
    project_id: Uuid,
    wh: Wh<Px>,
    cut: &Cut,
    opacity: OneZero,
    cg_files: &Vec<CgFile>,
) -> RenderingTree {
    let paint_builder = namui::PaintBuilder::new().set_color_filter(
        Color::from_f01(1.0, 1.0, 1.0, opacity.as_f32()),
        BlendMode::DstIn,
    );

    let graphics = cut.screen_graphics.iter().map(|(_, screen_graphic)| {
        render_graphic(
            project_id,
            wh,
            screen_graphic,
            Some(paint_builder.clone()),
            cg_files,
        )
    });
    render(graphics)
}

pub fn render_graphic(
    project_id: Uuid,
    wh: Wh<Px>,
    graphic: &ScreenGraphic,
    paint_builder: Option<PaintBuilder>,
    cg_files: &Vec<CgFile>,
) -> RenderingTree {
    namui::try_render(|| match graphic {
        ScreenGraphic::Image(screen_image) => {
            let url = get_project_image_url(project_id, screen_image.id).unwrap();
            let image = namui::image::try_load_url(&url)?;

            let rect =
                calculate_graphic_rect_on_screen(image.size(), wh, screen_image.circumscribed);

            Some(namui::image(ImageParam {
                rect,
                source: ImageSource::Image(image),
                style: ImageStyle {
                    fit: ImageFit::Fill,
                    paint_builder,
                },
            }))
        }
        ScreenGraphic::Cg(screen_cg) => {
            let url = get_project_cg_thumbnail_image_url(project_id, screen_cg.id).unwrap();
            let image = namui::image::try_load_url(&url)?;
            let outer_rect =
                calculate_graphic_rect_on_screen(image.size(), wh, screen_cg.circumscribed);

            let cg_file = cg_files
                .iter()
                .find(|cg_file| cg_file.name == screen_cg.name)?;
            Some(cg_render::render_cg(
                cg_render::CgRenderProps {
                    cg_id: screen_cg.id,
                    project_id,
                    rect: outer_rect,
                },
                screen_cg,
                cg_file,
            ))
        }
    })
}
