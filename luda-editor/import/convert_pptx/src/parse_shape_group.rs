use import::Text;
use msoffice_pptx::{
    document::PPTXDocument,
    pml::{Picture, Shape, ShapeGroup},
};
use namui_type::{Angle, PercentExt, Wh, Xywh};
use std::{
    fmt::Write,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::ppt_path::convert_ppt_path_to_url;

pub struct Context<'a> {
    pub page: import::Page,
    pub ppt: &'a PPTXDocument,
    pub slide_name: &'a str,
    pub slide_wh: Wh<u32>,
}

pub fn parse_shape_group(shape_group: &ShapeGroup, context: &mut Context) {
    match shape_group {
        msoffice_pptx::pml::ShapeGroup::Shape(shape) => {
            let text = extract_text(shape);
            if text.is_empty() {
                return;
            }
            context.page.texts.push(Text {
                content: text,
                font: import::Font::default(),
            });
        }
        msoffice_pptx::pml::ShapeGroup::Picture(picture) => {
            let Some(image) = extract_image(picture, context) else {
                return;
            };
            context.page.images.push(image);
        }
        msoffice_pptx::pml::ShapeGroup::GroupShape(group_shape) => {
            for shape_group in &group_shape.shape_array {
                parse_shape_group(shape_group, context);
            }
        }
        _ => {}
    }
}

fn extract_text(shape: &Shape) -> String {
    let mut text = String::new();
    let text_run_list = shape
        .text_body
        .iter()
        .flat_map(|text_body| text_body.paragraph_array.iter())
        .flat_map(|paragraph| paragraph.text_run_list.iter());
    for text_run in text_run_list {
        match text_run {
            msoffice_shared::drawingml::TextRun::RegularTextRun(regular_text_run) => {
                write!(text, "{}", regular_text_run.text).unwrap();
            }
            msoffice_shared::drawingml::TextRun::LineBreak(_) => {
                writeln!(text).unwrap();
            }
            msoffice_shared::drawingml::TextRun::TextField(text_field) => {
                let Some(text_field_text) = &text_field.text else {
                    continue;
                };
                write!(text, "{}", text_field_text).unwrap();
            }
        }
    }
    text
}

fn extract_image(picture: &Picture, context: &Context) -> Option<import::Image> {
    let Context {
        ppt,
        slide_name,
        slide_wh,
        ..
    } = context;

    let transform = picture.shape_props.transform.as_ref().unwrap();
    let angle = transform.rotate_angle.map_or(Angle::Degree(0.0), |angle| {
        Angle::Degree((angle / 60000) as f32)
    });

    let blip = picture.blip_fill.blip.as_ref().unwrap();
    let relation_index: usize = {
        let one_based_index: isize = blip
            .embed_rel_id
            .as_ref()
            .or(blip.linked_rel_id.as_ref())
            .unwrap()
            .trim_start_matches("rId")
            .parse()
            .unwrap();
        (one_based_index - 1) as usize
    };
    let relationship_path =
        PathBuf::from_str(&format!("ppt/slides/_rels/{}.xml.rels", slide_name)).unwrap();
    let relationships = ppt.slide_rels_map.get(&relationship_path).unwrap();
    let relationship = relationships.get(relation_index).unwrap();
    let ppt_image_path = format!(
        "ppt/media/{}",
        Path::new(&relationship.target)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
    );

    let offset = transform.offset.as_ref().unwrap();
    let extents = transform.extents.as_ref().unwrap();
    Some(import::Image {
        url: convert_ppt_path_to_url(&ppt_image_path),
        xywh: Xywh {
            x: (100.0 * offset.x as f32 / slide_wh.width as f32).percent(),
            y: (100.0 * offset.y as f32 / slide_wh.height as f32).percent(),
            width: (100.0 * extents.width as f32 / slide_wh.width as f32).percent(),
            height: (100.0 * extents.height as f32 / slide_wh.height as f32).percent(),
        },
        rotate: angle,
    })
}
