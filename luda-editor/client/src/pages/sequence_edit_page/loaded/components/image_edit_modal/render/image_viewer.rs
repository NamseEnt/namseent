use super::*;

pub struct Props {
    pub wh: Wh<Px>,
}

impl ImageEditModal {
    pub fn render_image_viewer(&self, props: Props) -> namui::RenderingTree {
        match self.image.as_ref() {
            Some(image) => namui::image(ImageParam {
                rect: Rect::zero_wh(props.wh),
                source: ImageSource::File(image.clone()),
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint_builder: None,
                },
            }),
            None => button::body_text_button(
                Rect::zero_wh(props.wh),
                "Click to upload",
                Color::WHITE,
                Color::WHITE,
                1.px(),
                Color::BLACK,
                TextAlign::Center,
                || {
                    spawn_local(async move {
                        let files = namui::file::picker::open().await;
                        let first_file = if files.len() > 0 {
                            files[0].clone()
                        } else {
                            return;
                        };
                        namui::event::send(InternalEvent::ImageChanged { image: first_file });
                    })
                },
            ),
        }
    }
}
