mod psd_parsing;

use anyhow::Result;
use include_dir::{include_dir, Dir};
use namui_type::Uuid;
// use opencv::prelude::*;
use image::imageops::FilterType;
use psd_parsing::{parse_psd, PsdParsingResult};
use rayon::prelude::*;
use rpc::{
    data::{CgFile, CgPart, CgPartVariant, Cut, Sequence},
    uuid,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

static PSDS_DIR: Dir<'_> = include_dir!("src/psds");
static IMAGES_DIR: Dir<'_> = include_dir!("src/images");
const CHARACTER_NAMES: [&str; 15] = [
    "오하연",
    "피디",
    "선임피디",
    "김혜진",
    "나지",
    "카페 직원",
    "MCN 담당자",
    "연습생들",
    "연습생",
    "연습생 1",
    "연습생 2",
    "프로듀서 1",
    "???",
    "댄스 트레이너",
    "피디들",
];

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CaseMetadata {
    data: BTreeMap<Uuid, PsdCase>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct PsdCase {
    case_id: Uuid,
    cg_file: CgFile,
    variants: Vec<CgPartVariant>,
    image_hash: u64,
}

fn main() -> Result<()> {
    let input = include_str!("input.json");
    let input: Input = serde_json::from_str(input).unwrap();

    let mut sequence = Sequence::new(uuid(), "0주차-0-카페".to_string());
    let psd_all_cases = get_psd_all_cases()?;

    // let psd_all_cases = psd_all_cases
    //     .into_par_iter()
    //     .map(|(mut case, image)| {
    //         let case_id = case.case_id;
    //         let file_path = format!("output/{case_id}.png");
    //         let hash = get_image_hash(file_path.as_str());

    //         case.image_hash = hash;

    //         (case, image)
    //     })
    //     .collect::<Vec<_>>();

    {
        let mut image_urls: BTreeSet<String> = BTreeSet::new();

        input.pages.iter().for_each(|x| {
            x.images.iter().for_each(|y| {
                image_urls.insert(y.url.clone());
            })
        });

        let result = image_urls
            // .into_par_iter()
            .into_iter()
            .map(|image_url| {
                let image_name = image_url.split('/').last().unwrap();
                let static_image = IMAGES_DIR
                    .get_file(&format!("{image_name}.png"))
                    .or_else(|| IMAGES_DIR.get_file(&format!("{image_name}.jpg")))
                    .or_else(|| IMAGES_DIR.get_file(&format!("{image_name}.gif")))
                    .expect(&format!("image not found: {image_name}"));

                // let image_hash =
                //     get_image_hash(&format!("src/images/{}", image_path.to_str().unwrap()));

                let image = image::load_from_memory_with_format(
                    static_image.contents(),
                    match static_image.path().extension().unwrap().to_str().unwrap() {
                        "png" => image::ImageFormat::Png,
                        "jpg" => image::ImageFormat::Jpeg,
                        _ => unreachable!(),
                    },
                )
                .unwrap();
                let width = 256;
                let height = 256;
                let image = image.resize_to_fill(width, height, FilterType::Nearest);
                let context = dssim::new();
                let image = context
                    .create_image(&imgref::ImgVec::new(
                        image.to_luma32f().into_raw(),
                        width as usize,
                        height as usize,
                    ))
                    .unwrap();

                let (distance, psd_case) = psd_all_cases
                    .par_iter()
                    .map(move |(case, psd_image)| {
                        let psd_image_dssim = context
                            .create_image(&imgref::ImgVec::new(
                                psd_image.to_luma32f().into_raw(),
                                width as usize,
                                height as usize,
                            ))
                            .unwrap();

                        let (distance, _) = context.compare(&image, &psd_image_dssim);

                        (f64::from(distance), case)
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .reduce(|(min_distance, min_case), (distance, case)| {
                        if distance < min_distance {
                            (distance, case)
                        } else {
                            (min_distance, min_case)
                        }
                    })
                    .unwrap();

                println!(
                    "{distance}] {case_id} <-> {image_url}",
                    case_id = psd_case.case_id,
                );
                (distance, psd_case, image_url)
            })
            .collect::<Vec<_>>();

        for (distance, PsdCase { case_id, .. }, image_url) in result {
            println!("{distance}] {case_id} <-> {image_url}");
        }
    };

    // for Page { images, texts } in input.pages {
    //     let mut cut = Cut::new(uuid());

    //     handle_texts(&mut cut, texts);
    //     handle_images(&mut cut, images);

    //     sequence.cuts.push(cut);
    // }

    // let sequence_json = serde_json::to_string_pretty(&sequence).unwrap();

    // fs::write("sequence.json", sequence_json).unwrap();

    // println!("{text_set:?}");

    Ok(())
}

// fn get_image_hash(path: &str) -> u64 {
//     let mut hash_output = opencv::core::Mat::default();
//     let mat =
//         opencv::imgcodecs::imread(path, 0).expect(format!("failed to read image {path}").as_str());
//     // opencv::img_hash::block_mean_hash(&mat, &mut hash_output, 0)
//     //     .expect(format!("failed to hash image {path}").as_str());
//     opencv::img_hash::p_hash(&mat, &mut hash_output)
//         .expect(format!("failed to hash image {path}").as_str());
//     hash_output
//         .data_bytes()
//         .unwrap()
//         .into_iter()
//         .fold(0, |acc, x| (acc << 8) | *x as u64)
// }

fn get_psd_all_cases() -> Result<Vec<(PsdCase, image::DynamicImage)>> {
    // let psds = PSDS_DIR
    //     .files()
    //     .par_bridge()
    //     .map(|psd_file| {
    //         parse_psd(
    //             psd_file.contents(),
    //             psd_file.path().file_name().unwrap().to_str().unwrap(),
    //         )
    //         .expect(format!("failed to parse psd: {:?}", psd_file.path()).as_str())
    //     })
    //     .collect::<Vec<_>>();

    // for psd in psds.iter() {
    //     if psd.variants_images.is_empty() {
    //         panic!("psd has no variants: {:?}", psd.cg_file);
    //     }
    // }

    // fs::create_dir_all("output")?;

    // let psd_all_cases: Vec<(PsdCase, image::DynamicImage)> =
    //     psds.into_par_iter()
    //         .flat_map(
    //             |PsdParsingResult {
    //                  variants_images,
    //                  cg_file,
    //                  wh,
    //              }| {
    //                 fn generate_all_cases(parts: Vec<CgPart>) -> Vec<Vec<CgPartVariant>> {
    //                     if parts.len() == 0 {
    //                         return vec![];
    //                     }
    //                     let (first_part, rest_parts) = parts.split_first().unwrap();
    //                     let variant_cases = variant_cases(first_part.clone());
    //                     let rest_parts_cases = generate_all_cases(rest_parts.to_vec());
    //                     if rest_parts_cases.is_empty() {
    //                         return variant_cases.into_iter().map(|x| vec![x]).collect();
    //                     }

    //                     variant_cases
    //                         .into_iter()
    //                         .flat_map(|variant| {
    //                             rest_parts_cases.clone().into_iter().map(
    //                                 move |mut rest_parts_case| {
    //                                     rest_parts_case.insert(0, variant.clone());
    //                                     rest_parts_case
    //                                 },
    //                             )
    //                         })
    //                         .collect::<Vec<_>>()
    //                 }

    //                 let all_cases = generate_all_cases(cg_file.parts.clone());

    //                 all_cases.into_par_iter().map(move |variants| {
    //                     let cg_file = cg_file.clone();

    //                     let layer_images = variants
    //                         .clone()
    //                         .into_par_iter()
    //                         .map(|variant| {
    //                             variants_images
    //                                 .iter()
    //                                 .find_map(|variants_image| {
    //                                     if variants_image.variant_id == variant.id {
    //                                         Some(&variants_image.image_buffer)
    //                                     } else {
    //                                         None
    //                                     }
    //                                 })
    //                                 .unwrap()
    //                         })
    //                         .collect::<Vec<_>>();

    //                     let mut bottom =
    //                         image::ImageBuffer::<image::Rgba<u8>, _>::new(wh.width, wh.height);

    //                     for part_image in layer_images.into_iter().rev() {
    //                         image::imageops::overlay(&mut bottom, part_image, 0, 0);
    //                     }

    //                     let case_id = namui_type::uuid_from_hash(
    //                         variants.iter().map(|x| x.id).collect::<Vec<_>>(),
    //                     );

    //                     let file_path = format!("output/{case_id}.png");
    //                     bottom.save(&file_path).unwrap();

    //                     // let image_hash = get_image_hash(&file_path);
    //                     let image_hash = 0;

    //                     (
    //                         PsdCase {
    //                             cg_file,
    //                             image_hash,
    //                             variants,
    //                             case_id,
    //                         },
    //                         bottom.into(),
    //                     )
    //                 })
    //             },
    //         )
    //         .collect::<Vec<_>>();

    // println!("psd_all_cases: {:?}", psd_all_cases.len());
    // let data = psd_all_cases
    //     .iter()
    //     .map(|(case, _image)| (case.case_id, case.clone()))
    //     .collect::<BTreeMap<_, _>>();

    // let case_metadata = CaseMetadata { data };
    // fs::write(
    //     "output/case_metadata.json",
    //     serde_json::to_string_pretty(&case_metadata)?,
    // )?;

    // panic!("done");

    let metadata: CaseMetadata =
        serde_json::from_str(&fs::read_to_string("output/case_metadata.json")?)?;

    let psd_all_cases = metadata
        .data
        .into_par_iter()
        .map(|(uuid, case)| {
            let buffer = image::open(format!("output/{uuid}.png"))
                .unwrap()
                .resize_to_fill(256, 256, FilterType::Nearest);
            (case, buffer.to_luma32f().into())
        })
        .collect::<Vec<_>>();

    Ok(psd_all_cases)
}

fn find_psd_for_image(raw_image_buffer: &[u8], psds: &[PsdParsingResult]) -> Result<()> {
    let image = image::load_from_memory(raw_image_buffer)?;
    // todo parse image
    // todo check size
    psds.into_par_iter().find_any(|psd| {
        // let psd_pixel_count = {
        //     let first_layer = &psd.variants_bitmaps.first().unwrap().1;
        //     first_layer.len() / 4
        // };
        // let image_pixel_count = image.width() * image.height();
        // if psd_pixel_count != image_pixel_count as usize {
        //     println!("psd_pixel_count: {psd_pixel_count}, image_pixel_count: {image_pixel_count}");
        //     return false;
        // }

        fn generate_all_cases(parts: Vec<CgPart>) -> Vec<Vec<CgPartVariant>> {
            if parts.len() == 0 {
                return vec![];
            }
            let (first_part, rest_parts) = parts.split_first().unwrap();
            let variant_cases = variant_cases(first_part.clone());
            let rest_parts_cases = generate_all_cases(rest_parts.to_vec());

            variant_cases
                .into_iter()
                .flat_map(|variant| {
                    rest_parts_cases
                        .clone()
                        .into_iter()
                        .map(move |mut rest_parts_case| {
                            rest_parts_case.insert(0, variant.clone());
                            rest_parts_case
                        })
                })
                .collect::<Vec<_>>()
        }

        let all_cases = generate_all_cases(psd.cg_file.parts.clone());

        todo!()
    });

    Ok(())
}

fn variant_cases(part: CgPart) -> Vec<CgPartVariant> {
    match part.selection_type {
        rpc::data::PartSelectionType::Single => part.variants,
        rpc::data::PartSelectionType::Multi => {
            todo!()
        }
        rpc::data::PartSelectionType::AlwaysOn => part.variants,
    }
}

fn handle_images(cut: &mut Cut, images: Vec<Image>) {
    todo!()
}

fn handle_texts(cut: &mut Cut, texts: Vec<Text>) {
    if texts.len() == 0 {
        // nothing
    } else if texts.len() >= 2 {
        let mut it = texts.into_iter();
        let character = it.next().unwrap();
        let texts = it.collect::<Vec<_>>();
        assert!(CHARACTER_NAMES.contains(&character.content.as_ref()));

        cut.character_name = character.content;

        cut.line = texts
            .into_iter()
            .map(|text| text.content)
            .collect::<Vec<_>>()
            .join("\n");
    } else {
        cut.line = texts
            .into_iter()
            .map(|text| text.content)
            .collect::<Vec<_>>()
            .join("\n");
    }
}

#[derive(serde::Deserialize, Debug)]
struct Input {
    pages: Vec<Page>,
}

#[derive(serde::Deserialize, Debug)]
struct Page {
    images: Vec<Image>,
    texts: Vec<Text>,
}

#[derive(serde::Deserialize, Debug)]
struct Image {
    url: String,
    xywh: Xywh,
}

#[derive(serde::Deserialize, Debug)]
struct Xywh {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[derive(serde::Deserialize, Debug)]
struct Text {
    content: String,
    font: Font,
}

#[derive(serde::Deserialize, Debug)]
struct Font {
    size: usize,
    weight: usize,
    family: String,
    bold: bool,
    italic: bool,
    strikethrough: bool,
    underline: bool,
}
