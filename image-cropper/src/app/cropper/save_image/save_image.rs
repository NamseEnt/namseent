use super::{
    create_element::{create_anchor_element, create_canvas_element, create_image_element},
    get_png_encoded_u8_from_canvas::get_png_encoded_u8_from_canvas,
    image_zipper::ImageZipper,
};
use crate::app::cropper::selection::Selection;
use js_sys::{Array, Uint8Array};
use namui::{LtrbRect, Xy, XywhRect};
use std::future::Future;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Blob, CanvasRenderingContext2d, Event, HtmlCanvasElement, HtmlImageElement, Url};

pub fn save_image(image_url: String, image_name: String, selection_list: Vec<Selection>) {
    let canvas = create_canvas_element();
    let context = get_context(&canvas);
    let image_element = create_image_element();

    add_onload_event_listener_to_imgae_element(&image_element, move |target| async move {
        let mut image_zipper = ImageZipper::new();
        for (index, selection) in selection_list.into_iter().enumerate() {
            let polygon = selection.get_polygon();
            if let Some(bounding_box) = get_bounding_box(&polygon) {
                canvas.set_width(bounding_box.width as u32);
                canvas.set_height(bounding_box.height as u32);
                context.save();
                context
                    .translate(-bounding_box.x as f64, -bounding_box.y as f64)
                    .expect("failed to translate");
                draw_clip_path_with_polygon(&context, &polygon);
                draw_selected_image(&context, &target, &bounding_box);
                context.restore();
                let image_name = make_sequential_file_name(&image_name, index);
                let png_encoded_u8 = get_png_encoded_u8_from_canvas(&canvas).await;
                image_zipper.add_image(image_name.as_str(), png_encoded_u8);
            }
        }
        let zip_vec_u8 = image_zipper.finish();
        download_vec_u8(zip_vec_u8, format!("{image_name}.zip"));
    });
    image_element.set_src(image_url.as_str());
}

fn draw_selected_image(
    context: &CanvasRenderingContext2d,
    image_element: &HtmlImageElement,
    selection_bounding_box: &XywhRect<f32>,
) {
    context
        .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &image_element,
            selection_bounding_box.x.into(),
            selection_bounding_box.y.into(),
            selection_bounding_box.width.into(),
            selection_bounding_box.height.into(),
            selection_bounding_box.x.into(),
            selection_bounding_box.y.into(),
            selection_bounding_box.width.into(),
            selection_bounding_box.height.into(),
        )
        .expect("failed to draw image to offscreen canvas");
}

fn add_onload_event_listener_to_imgae_element<CB, FT>(
    image_element: &HtmlImageElement,
    callback: CB,
) where
    CB: FnOnce(HtmlImageElement) -> FT + 'static,
    FT: Future<Output = ()>,
{
    image_element
        .add_event_listener_with_callback(
            "load",
            Closure::once(Box::new(move |event: Event| {
                spawn_local(async move {
                    let target = event
                        .target()
                        .expect("image element not found")
                        .dyn_into::<HtmlImageElement>()
                        .expect("failed to cast image element");
                    callback(target).await;
                })
            }) as Box<dyn FnOnce(Event)>)
            .into_js_value()
            .as_ref()
            .unchecked_ref(),
        )
        .expect("failed to attach onload listener");
}

fn make_sequential_file_name(original_name: &String, index: usize) -> String {
    let mut name = original_name.clone();
    let most_right_dot = name.rfind(".").unwrap_or(name.len());
    name.insert_str(most_right_dot, format!("_{:02}", index).as_str());
    if !name.ends_with(".png") {
        name.push_str(".png")
    }
    name
}

fn download_vec_u8(vec_u8: Vec<u8>, file_name: String) {
    let u8_array = Uint8Array::from(vec_u8.as_ref());
    let u8_array_sequence = Array::new();
    u8_array_sequence.push(&u8_array);
    let blob = Blob::new_with_buffer_source_sequence(&u8_array_sequence).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    download_url(url, file_name)
}

fn download_url(url: String, name: String) {
    let anchor_element = create_anchor_element();
    anchor_element.set_download(name.as_str());
    anchor_element.set_href(url.as_str());
    anchor_element.click();
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

fn draw_clip_path_with_polygon(context: &CanvasRenderingContext2d, polygon: &Vec<Xy<f32>>) {
    if let Some(first_point) = polygon.first() {
        context.begin_path();
        context.move_to(first_point.x as f64, first_point.y as f64);
        for point in polygon.iter().skip(1) {
            context.line_to(point.x as f64, point.y as f64);
        }
        context.close_path();
        context.clip();
    }
}
