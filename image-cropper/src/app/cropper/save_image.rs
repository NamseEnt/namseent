use super::selection::Selection;
use namui::{LtrbRect, Xy, XywhRect};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{
    window, Blob, CanvasRenderingContext2d, Event, HtmlAnchorElement, HtmlCanvasElement,
    HtmlImageElement, Url,
};

pub fn save_image(image_url: String, image_name: String, selection_list: Vec<Selection>) {
    let canvas = create_canvas_element();
    let context = get_context(&canvas);
    let image_element = create_image_element();
    image_element
        .add_event_listener_with_callback(
            "load",
            Closure::once(Box::new(move |event: Event| {
                let target = event.target().expect("image element not found")
                .dyn_into::<HtmlImageElement>()
                .expect("failed to cast image element");
                selection_list.into_iter().enumerate().for_each(|(index, selection)| {
                    let polygon = selection.get_polygon();
                    if let Some(bounding_box) = get_bounding_box(&polygon) {
                        canvas.set_width(bounding_box.width as u32);
                        canvas.set_height(bounding_box.height as u32);
                        context
                            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                                &target,
                                bounding_box.x.into(),
                                bounding_box.y.into(),
                                bounding_box.width.into(),
                                bounding_box.height.into(),
                                0.0,
                                0.0,
                                bounding_box.width.into(),
                                bounding_box.height.into(),
                            )
                            .expect("failed to draw image to offscreen canvas");
                        save_canvas_to_png(&canvas, make_sequential_file_name(&image_name, index + 1));
                    }
                })
            }) as Box<dyn FnOnce(Event)>)
            .into_js_value()
            .as_ref()
            .unchecked_ref(),
        )
        .expect("failed to attach onload listener");
    image_element.set_src(image_url.as_str());
}

fn make_sequential_file_name(original_name: &String, index: usize) -> String {
    let mut name = original_name.clone();
    let most_right_dot = name.rfind(".").unwrap_or(name.len());
    name.insert_str(most_right_dot, format!("_{:02}", index).as_str());
    name
}

fn save_canvas_to_png(canvas: &HtmlCanvasElement, name: String) {
    canvas
        .to_blob_with_type(
            Closure::once(Box::new(move |blob: Blob| {
                Url::create_object_url_with_blob(&blob)
                    .and_then(|url| Ok(download_url(url, name)))
                    .expect("failed to download canvas to png");
            }) as Box<dyn FnOnce(Blob)>)
            .into_js_value()
            .as_ref()
            .unchecked_ref(),
            "image/png",
        )
        .expect("failed to create blob url from canvas")
}

fn download_url(url: String, name: String) {
    let anchor_element = create_anchor_element();
    anchor_element.set_download(name.as_str());
    anchor_element.set_href(url.as_str());
    anchor_element.click();
}

fn create_anchor_element() -> HtmlAnchorElement {
    let document = window()
        .expect("failed to get window")
        .document()
        .expect("failed to get window.document");
    let element = document
        .create_element("a")
        .expect("failed to create HTMLAnchorElement");
    let anchor_element = wasm_bindgen::JsCast::dyn_into::<HtmlAnchorElement>(element)
        .expect("failed to cast HTMLAnchorElement");
    anchor_element
}

fn get_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .expect("failed to get renderingContext2d of offscreen canvas")
        .expect("offscreen canvas didn't return context")
        .dyn_into()
        .expect("failed to cast renderingContext2d");
    context
}

fn create_canvas_element() -> HtmlCanvasElement {
    let document = window()
        .expect("failed to get window")
        .document()
        .expect("failed to get window.document");
    let element = document
        .create_element("canvas")
        .expect("failed to create HTMLCanvasElement");
    let canvas_element = wasm_bindgen::JsCast::dyn_into::<HtmlCanvasElement>(element)
        .expect("failed to cast HTMLCanvasElement");
    canvas_element
}

fn create_image_element() -> HtmlImageElement {
    let document = window()
        .expect("failed to get window")
        .document()
        .expect("failed to get window.document");
    let element = document
        .create_element("img")
        .expect("failed to create HTMLImageElement");
    let image_element = wasm_bindgen::JsCast::dyn_into::<HtmlImageElement>(element)
        .expect("failed to cast HTMLImageElement");
    image_element
}

fn get_bounding_box(polygon: &Vec<Xy<f32>>) -> Option<XywhRect<f32>> {
    polygon
        .first()
        .and_then(|first_point| {
            Some(LtrbRect {
                left: first_point.x,
                top: first_point.y,
                right: first_point.x,
                bottom: first_point.y,
            })
        })
        .and_then(|initial_bounding_box| {
            let bounding_box =
                polygon
                    .into_iter()
                    .fold(initial_bounding_box, |bounding_box: LtrbRect, point| {
                        LtrbRect {
                            left: bounding_box.left.min(point.x),
                            top: bounding_box.top.min(point.y),
                            right: bounding_box.right.max(point.x),
                            bottom: bounding_box.bottom.max(point.y),
                        }
                    });
            Some(XywhRect {
                x: bounding_box.left,
                y: bounding_box.top,
                width: bounding_box.right - bounding_box.left,
                height: bounding_box.bottom - bounding_box.top,
            })
        })
}
