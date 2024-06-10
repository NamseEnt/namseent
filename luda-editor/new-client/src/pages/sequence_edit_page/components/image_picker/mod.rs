use crate::{
    color,
    pages::sequence_edit_page::atom::{IMAGES_ATOM, SEQUENCE_ATOM},
    storage::get_project_image_url,
};
use namui::*;
use namui_prebuilt::{table::hooks::TableCell, *};
use rpc::data::{Cut, ImageWithLabels};

const OUTER_PADDING: Px = px(8.0);
const INNER_PADDING: Px = px(4.0);
const THUMBNAIL_WH: Wh<Px> = Wh {
    width: px(144.0),
    height: px(144.0),
};

#[namui::component]
pub struct ImagePicker<'a> {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
    pub cut: &'a Cut,
    pub on_event: &'a dyn Fn(Event),
}

pub enum Event {
    Close,
}
pub enum InternalEvent {
    ImageThumbnailClicked { image_id: Uuid },
}

impl Component for ImagePicker<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self {
            wh,
            project_id,
            cut,
            on_event,
        } = self;
        let (images, _set_images) = ctx.atom(&IMAGES_ATOM);

        let on_internal_event = |event: InternalEvent| match event {
            InternalEvent::ImageThumbnailClicked { image_id } => {
                add_image_to_cut(image_id, cut.id);
                on_event(Event::Close);
            }
        };

        ctx.compose(|ctx| {
            table::hooks::padding(OUTER_PADDING, |wh, ctx| {
                let max_items_per_row = (wh.width / (THUMBNAIL_WH.width)).floor() as usize;
                ctx.add(scroll_view::AutoScrollViewWithCtx {
                    scroll_bar_width: 4.px(),
                    wh,
                    content: |ctx| {
                        table::hooks::vertical(images.chunks(max_items_per_row).map(|images| {
                            table::hooks::fixed(THUMBNAIL_WH.height, {
                                table::hooks::horizontal(images.iter().map(|image| {
                                    render_thumbnail(image, project_id, &on_internal_event)
                                }))
                            })
                        }))(wh, ctx)
                    },
                });
            })(wh, ctx)
        });

        let background = simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND)
            .attach_event(|event| {
                if let namui::Event::MouseDown { event } = event {
                    event.stop_propagation();
                    if !event.is_local_xy_in() {
                        on_event(Event::Close);
                    }
                }
            });

        ctx.component(background);

        
    }
}

fn render_thumbnail<'a>(
    image: &'a ImageWithLabels,
    project_id: Uuid,
    on_internal_event: &'a dyn Fn(InternalEvent),
) -> TableCell<'a> {
    table::hooks::fixed::<'a>(THUMBNAIL_WH.width, {
        table::hooks::padding(INNER_PADDING, move |wh, ctx| {
            ctx.add(
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT)
                    .with_mouse_cursor(MouseCursor::Pointer)
                    .attach_event({
                        |event| {
                            if let namui::Event::MouseDown { event } = event {
                                if event.is_local_xy_in() {
                                    on_internal_event(InternalEvent::ImageThumbnailClicked {
                                        image_id: image.id,
                                    });
                                }
                            }
                        }
                    }),
            )
            .add(get_project_image_url(project_id, image.id).map_or(
                RenderingTree::Empty,
                |cg_thumbnail_image_url| {
                    namui::image(ImageParam {
                        rect: Rect::from_xy_wh(Xy::zero(), wh),
                        source: ImageSource::Url {
                            url: cg_thumbnail_image_url,
                        },
                        style: ImageStyle {
                            fit: ImageFit::Contain,
                            paint: None,
                        },
                    })
                },
            ));
        })
    })
}

fn add_image_to_cut(image_id: Uuid, cut_id: Uuid) {
    SEQUENCE_ATOM.mutate(move |sequence| {
        sequence.update_cut(
            cut_id,
            rpc::data::CutUpdateAction::PushScreenGraphic {
                graphic_index: uuid(),
                screen_graphic: rpc::data::ScreenGraphic::Image(rpc::data::ScreenImage::new(
                    image_id,
                )),
            },
        )
    })
}
