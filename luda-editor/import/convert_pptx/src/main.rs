mod find_same_existing_image;
mod get_image_hash;
mod parse_shape_group;
mod ppt_path;

use find_same_existing_image::find_same_existing_image;
use import::{Input, Page};
use msoffice_pptx::document::PPTXDocument;
use namui_type::Wh;
use parse_shape_group::{parse_shape_group, Context};
use ppt_path::{convert_ppt_path_to_url, convert_url_to_ppt_path};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};
use zip::ZipArchive;

fn main() {
    let ppt_path = Path::new("input.pptx");
    let mut zipper = ZipArchive::new(File::open(ppt_path).unwrap()).unwrap();
    let ppt = PPTXDocument::from_file(ppt_path).unwrap();
    let slide_wh = {
        let slide_size = ppt
            .presentation
            .as_ref()
            .unwrap()
            .slide_size
            .as_ref()
            .unwrap();
        Wh::new(slide_size.width, slide_size.height)
    };
    let mut input = Input { pages: Vec::new() };

    for slide_index in 1..ppt.slide_map.len() + 1 {
        let slide_name = format!("slide{}", slide_index);
        let slide_path = PathBuf::from_str(&format!("ppt/slides/{slide_name}.xml")).unwrap();
        let slide = ppt.slide_map.get(&slide_path).unwrap();

        let mut context = Context {
            page: Page {
                images: Vec::new(),
                texts: Vec::new(),
            },
            ppt: &ppt,
            slide_name: &slide_name,
            slide_wh,
        };
        for shape_group in slide.common_slide_data.shape_tree.shape_array.iter() {
            parse_shape_group(shape_group, &mut context);
        }

        input.pages.push(context.page);
    }

    for page in input.pages.iter_mut() {
        for image in page.images.iter_mut() {
            let ppt_image_path = PathBuf::from_str(&convert_url_to_ppt_path(&image.url)).unwrap();
            let image_name = PathBuf::from_str(
                find_same_existing_image(&ppt_image_path, &mut zipper).image_name(),
            )
            .unwrap();
            let image_num = image_name.file_stem().unwrap().to_str().unwrap();
            image.url = convert_ppt_path_to_url(image_num);
        }
    }

    File::create("input.json")
        .unwrap()
        .write_all(serde_json::to_string_pretty(&input).unwrap().as_bytes())
        .unwrap();
}
