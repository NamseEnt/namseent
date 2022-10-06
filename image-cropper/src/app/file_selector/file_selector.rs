use super::{render_file_select_dialog_open_button, FileSelectorEvent};
use crate::app::{cropper::Cropper, router::RouterEvent, util::alert};
use js_sys::Uint8Array;
use namui::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{window, FileList, HtmlInputElement, InputEvent, Url};

pub struct FileSelectorProps {
    pub screen_wh: Wh<Px>,
}

pub struct FileSelector {
    html_input_element: HtmlInputElement,
}
impl FileSelector {
    pub fn new() -> Self {
        let input_element = create_image_input_element();
        set_file_handler(
            &input_element,
            |file, url, name| match namui::image::new_image_from_u8(&file) {
                Some(image) => {
                    namui::event::send(FileSelectorEvent::NamuiImagePrepared { image, url, name })
                }
                None => namui::event::send(FileSelectorEvent::NamuiImageMakeFailed(format!(
                    "failed to make image of {}",
                    name
                ))),
            },
        );

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
        const MARGIN: Px = px(16.0);
        const BUTTON_HEIGHT: Px = px(36.0);
        let button_wh = Wh {
            width: props.screen_wh.width - (MARGIN * 2.0),
            height: BUTTON_HEIGHT,
        };
        render([
            render_background(props.screen_wh),
            translate(
                MARGIN,
                (props.screen_wh.height - BUTTON_HEIGHT) / 2.0,
                render_file_select_dialog_open_button(button_wh),
            ),
        ])
    }
}

fn render_background(wh: Wh<Px>) -> RenderingTree {
    namui::rect(RectParam {
        rect: Rect::Xywh {
            x: px(0.0),
            y: px(0.0),
            width: wh.width,
            height: wh.height,
        },
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
            .unchecked_ref(),
        )
        .expect("failed to attach file handler");
}
