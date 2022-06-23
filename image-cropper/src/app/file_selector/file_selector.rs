use super::{render_file_select_dialog_open_button, FileSelectorEvent};
use crate::app::{cropper::Cropper, router::RouterEvent, util::alert};
use js_sys::Uint8Array;
use namui::{
    render, translate, Color, Image, RectFill, RectParam, RectStyle, RenderingTree, Wh, CANVAS_KIT,
};
use std::sync::Arc;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{window, FileList, HtmlInputElement, InputEvent, Url};

pub struct FileSelectorProps {
    pub screen_wh: Wh<f32>,
}

pub struct FileSelector {
    html_input_element: HtmlInputElement,
}
impl FileSelector {
    pub fn new() -> Self {
        let input_element = create_image_input_element();
        set_file_handler(&input_element, |file, url, name| {
            match make_namui_image(file) {
                Ok(image) => namui::event::send(FileSelectorEvent::NamuiImagePrepared {
                    image: Arc::new(image),
                    url,
                    name,
                }),
                Err(error) => namui::event::send(FileSelectorEvent::NamuiImageMakeFailed(error)),
            }
        });

        Self {
            html_input_element: input_element,
        }
    }

    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<FileSelectorEvent>() {
            match &event {
                FileSelectorEvent::FileSelectDialogOpenButtonClicked => {
                    self.html_input_element.click()
                }
                FileSelectorEvent::NamuiImageMakeFailed(message) => {
                    alert(format!("failed to make image: {}", message).as_str())
                }
                FileSelectorEvent::NamuiImagePrepared { image, url, name } => {
                    let image = image.clone();
                    let url = url.clone();
                    let name = name.clone();
                    namui::event::send(RouterEvent::PageChangeRequestedToCropperEvent(Box::new(
                        move || {
                            let image = image.clone();
                            let url = url.clone();
                            let name = name.clone();
                            Cropper::new(image, url, name)
                        },
                    )))
                }
            }
        }
    }

    pub fn render(&self, props: &FileSelectorProps) -> RenderingTree {
        const MARGIN: f32 = 16.0;
        const BUTTON_HEIGHT: f32 = 36.0;
        let button_wh = Wh {
            width: props.screen_wh.width - (2.0 * MARGIN),
            height: BUTTON_HEIGHT,
        };
        render([
            render_background(&props.screen_wh),
            translate(
                MARGIN,
                (props.screen_wh.height - BUTTON_HEIGHT) / 2.0,
                render_file_select_dialog_open_button(button_wh),
            ),
        ])
    }
}

fn render_background(wh: &Wh<f32>) -> RenderingTree {
    namui::rect(RectParam {
        x: 0.0,
        y: 0.0,
        width: wh.width,
        height: wh.height,
        style: RectStyle {
            stroke: None,
            fill: Some(RectFill {
                color: Color::from_u8(36, 37, 42, 255),
            }),
            round: None,
        },
    })
}

fn create_image_input_element() -> HtmlInputElement {
    let document = window()
        .expect("failed to get window")
        .document()
        .expect("failed to get window.document");
    let element = document
        .create_element("input")
        .expect("failed to create HTMLInputElement");
    let input_element = wasm_bindgen::JsCast::dyn_into::<HtmlInputElement>(element)
        .expect("failed to cast HTMLInputElement");
    input_element.set_type("file");
    input_element.set_accept("image/*");
    input_element
}

fn set_file_handler(element: &HtmlInputElement, handler: fn(Vec<u8>, String, String)) {
    element
        .add_event_listener_with_callback(
            "change",
            Closure::wrap(Box::new(move |event: InputEvent| {
                let target = event
                    .target()
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap();
                if let Some(files) = target.files() {
                    if let Ok(files) = files.dyn_into::<FileList>() {
                        if let Some(file) = files.item(0) {
                            spawn_local(async move {
                                let uint8array = Uint8Array::new(
                                    &JsFuture::from(file.array_buffer())
                                        .await
                                        .expect("failed to read File as ArrayBuffer"),
                                );
                                let name = file.name();
                                let url = Url::create_object_url_with_blob(file.as_ref())
                                    .expect("Failed to create object url of image");
                                handler(uint8array.to_vec(), url, name);
                            })
                        }
                    }
                }
            }) as Box<dyn FnMut(InputEvent)>)
            .into_js_value()
            .as_ref()
            .unchecked_ref(),
        )
        .expect("failed to attach file handler");
}

fn make_namui_image(u8: impl AsRef<[u8]>) -> Result<Image, String> {
    match CANVAS_KIT.get() {
        Some(canvas_kit) => match canvas_kit.MakeImageFromEncoded(u8.as_ref()) {
            Some(canvas_kit_image) => Ok(Image::new(canvas_kit_image)),
            None => Err(format!("failed to MakeImageFromEncoded")),
        },
        None => Err(format!("failed to get canvas kit")),
    }
}
