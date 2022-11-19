use crate::storage::get_project_image_url;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::*;

pub struct Props<'a, OnClick: Fn(usize, &MouseEvent) + 'static + Copy> {
    pub wh: Wh<Px>,
    pub cut: &'a Cut,
    pub project_id: Uuid,
    pub on_click: OnClick,
    pub selected_index: Option<usize>,
}

pub fn render<'a, OnClick: Fn(usize, &MouseEvent) + 'static + Copy>(
    props: Props<'a, OnClick>,
) -> RenderingTree {
    namui::render([
        simple_rect(props.wh, Color::WHITE, 1.px(), Color::BLACK),
        table::horizontal(props.cut.screen_images.iter().enumerate().map(
            |(index, screen_image)| {
                table::ratio(1.0, move |wh| {
                    let image_source = screen_image.as_ref().map(|screen_image| {
                        get_project_image_url(props.project_id, screen_image.id).unwrap()
                    });

                    let border_color = match props.selected_index {
                        Some(selected_index) => {
                            if selected_index == index {
                                Color::RED
                            } else {
                                Color::grayscale_f01(0.5)
                            }
                        }
                        None => Color::WHITE,
                    };

                    let border_width = match props.selected_index {
                        Some(selected_index) if selected_index == index => 2.px(),
                        _ => 1.px(),
                    };

                    namui::render([
                        simple_rect(wh, border_color, border_width, Color::BLACK),
                        match image_source {
                            Some(image_source) => namui::image(ImageParam {
                                rect: Rect::from_xy_wh(Xy::single(0.px()), wh),
                                source: namui::ImageSource::Url(image_source),
                                style: ImageStyle {
                                    fit: ImageFit::Contain,
                                    paint_builder: None,
                                },
                            }),
                            None => RenderingTree::Empty,
                        },
                    ])
                    .attach_event(move |builder| {
                        builder.on_mouse_down_in(move |event| {
                            (props.on_click)(index, event);
                        });
                    })
                })
            },
        ))(props.wh),
    ])
}
