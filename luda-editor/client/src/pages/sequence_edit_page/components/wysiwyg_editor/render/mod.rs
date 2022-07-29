// pub mod cropper;
pub mod resizer;

use super::*;
use namui_prebuilt::*;

impl WysiwygEditor {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        render([
            simple_rect(props.wh, Color::WHITE, 1.px(), Color::TRANSPARENT),
            self.render_image_clip(&props),
        ])
    }
    fn render_image_clip(&self, props: &Props) -> RenderingTree {
        /*
            레이어별로 렌더링 시작.
            만약 해당 레이어가 선택되었으면 그거 수정하는 도구가 뜨도록 함.
        */
        render(props.image_clip.images.iter().enumerate().map(|(layer_index, image)| {
            let is_selected_layer = props.selected_layer_index == Some(layer_index);

            namui::try_render(|| {
                let url = namui::Url::parse(&format!("https://raw.githubusercontent.com/namseent/luda-editor-storage/master/resources/{}", image.image_path.as_ref()?)).unwrap();
                let namui_image = namui::image::try_load(&url)?;
                let image_size = namui_image.size();
                

                let mut image_size_on_screen = Wh::new(
                    image_size.width * image.circumscribed.radius / 1920.px() * props.wh.width,
                    image_size.height * image.circumscribed.radius / 1080.px() * props.wh.height,
                );

                if let Some(Dragging::Resizer { context }) =
                    self.dragging.as_ref()
                {
                    if is_selected_layer {
                        namui::log!("image_size_on_screen: {:?}", image_size_on_screen);
                        namui::log!("circumscribed: {:?}", image.circumscribed);
                        let circumscribed = context.resize(image_size_on_screen, props.wh);
                        image_size_on_screen = Wh::new(
                            image_size.width * circumscribed.radius / 1920.px() * props.wh.width,
                            image_size.height * circumscribed.radius / 1080.px() * props.wh.height,
                        );
                        namui::log!("after image_size_on_screen: {:?}", image_size_on_screen);
                        namui::log!("after circumscribed: {:?}", circumscribed);
                    }
                }

                let image_center_xy = Xy::new(
                    props.wh.width * image.circumscribed.center.x,
                    props.wh.height * image.circumscribed.center.y,
                );
                let image_left_top_xy = image_center_xy - image_size_on_screen.as_xy() / 2.0;

                let image_dest_rect = Rect::from_xy_wh(image_left_top_xy, image_size_on_screen);

                let wysiwyg_tool = if is_selected_layer {
                    self.render_wysiwyg_tool(props, image_dest_rect, image_size, layer_index)
                } else {
                    RenderingTree::Empty
                };

                Some(
                    render([
                        namui::image(ImageParam {
                            rect: image_dest_rect,
                            source: namui::ImageSource::Image(namui_image),
                            style: ImageStyle {
                                fit: ImageFit::None,
                                paint_builder: None,
                            },
                        }),
                        wysiwyg_tool,
                    ])
                )
            })
        }))
    }
    fn render_wysiwyg_tool(
        &self,
        props: &Props,
        image_dest_rect: Rect<Px>,
        image_size: Wh<Px>,
        layer_index: usize,
    ) -> RenderingTree {
        render([
            resizer::render_resizer(resizer::Props {
                rect: image_dest_rect,
                dragging_context: if let Some(Dragging::Resizer { context }) =
                    self.dragging.as_ref()
                {
                    Some(*context)
                } else {
                    None
                },
                on_resize: {
                    let image_clip_address = props.image_clip_address.clone();
                    Box::new(move |circumscribed| {
                        namui::log!("circumscribed: {:?}", circumscribed);
                        namui::event::send(Event::Resize {
                            circumscribed,
                            image_clip_address: image_clip_address.clone(),
                            layer_index,
                        })
                    })
                },
                container_size: props.wh,
                image_size,
            }),
            // self.render_cropper(props),
        ])
    }
}
