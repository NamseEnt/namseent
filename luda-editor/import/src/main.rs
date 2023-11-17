mod additional_graphic;
mod predetermined_graphic;

use anyhow::Result;
use include_dir::{include_dir, Dir};
use namui_type::{percent, Percent, Uuid, Xy, Wh, Angle};
use server_core::apis::cg::shared::{psd_to_cg_file::PsdParsingResult, layer_tree::RenderResult};
// use opencv::prelude::*;
use crate::{
    additional_graphic::push_additional_graphic_map,
    predetermined_graphic::get_predetermined_graphic_map,
};
use image::{imageops::FilterType, DynamicImage, ImageBuffer, Luma};
use predetermined_graphic::PredeterminedGraphic;
use rayon::prelude::*;
use rpc::{
    data::{
        CgFile, CgPart, Circumscribed, Cut, ScreenCg, ScreenCgPart, ScreenGraphic, ScreenImage,
        Sequence,
    },
    uuid,
};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fs::{self, DirEntry},
    io::ErrorKind,
    path::PathBuf,
    str::FromStr,
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
const BACKGROUND_IMAGE_DISTANCE_THRESHOLD: f64 = 0.85;
///
/// 0. Generate psd cases and save them to `output/case_metadata.json`, `output/{case_uuid}`.
/// 1. Measure distance between psd cases and input image and save them to `src/distance_psd_image_triples.json`.
/// 2. Generate predetermined_graphic_map and save it to `src/predetermined_graphic/predetermined_graphic_map.json`.
/// 3. Generate sequence and save it to `sequence.json`.
const CHECKPOINT: usize = 3;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CaseMetadata {
    data: BTreeMap<Uuid, PsdCase>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct PsdCase {
    case_id: Uuid,
    cg_file: CgFile,
    parts: Vec<ScreenCgPart>,
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

    // list_similar_psd_cases(&psd_all_cases, "1");

    let distance_psd_image_triples = get_distance_psd_image_triples(&input, &psd_all_cases);

    let predetermined_graphic_map = get_predetermined_graphic_map(&psd_all_cases);

    let image_url_psd_case_map = {
        let mut image_url_psd_case_map = HashMap::<String, (f64, &PsdCase, Uuid)>::new();
        for (distance, psd_case, image_url, image_id) in distance_psd_image_triples.into_iter() {
            match image_url_psd_case_map.get(&image_url) {
                Some((previous_distance, _, _)) => {
                    if distance < *previous_distance {
                        image_url_psd_case_map.insert(image_url, (distance, psd_case, image_id));
                    }
                }
                None => {
                    image_url_psd_case_map.insert(image_url, (distance, psd_case, image_id));
                }
            }
        }
        image_url_psd_case_map
    };

    let mut used_background_images = HashMap::<Uuid, String>::new();
    let mut used_cg_file_names = BTreeSet::<String>::new();
    for Page { images, texts } in input.pages {
        let mut cut = Cut::new(uuid());

        handle_texts(&mut cut, texts);
        handle_images(
            &mut cut,
            images,
            &image_url_psd_case_map,
            &predetermined_graphic_map,
            &mut used_background_images,
            &mut used_cg_file_names,
        );

        sequence.cuts.push(cut);
    }
    push_additional_graphic_map(&mut sequence);

    println!("Used background image urls, please upload them to project as image");
    let mut used_background_image_names = used_background_images
        .into_values()
        .collect::<Vec<_>>();
    used_background_image_names.sort();
    for image_name in used_background_image_names.iter() {
        println!("{image_name}");
    }

    println!();
    println!("Used cg file names, please upload them to project as cg file");
    let used_cg_file_names: Vec<_> = used_cg_file_names.into_iter().collect();
    for cg_file_name in used_cg_file_names.iter() {
        println!("{cg_file_name}");
    }

    let sequence_json = serde_json::to_string_pretty(&sequence).unwrap();

    fs::write("sequence.json", sequence_json).unwrap();

    copy_used_assets(&used_background_image_names, &used_cg_file_names);

    Ok(())
}

fn list_similar_psd_cases(psd_all_cases: &[(PsdCase, DynamicImage)], image_name: &str) {
    let static_image = IMAGES_DIR
        .get_file(&format!("{image_name}.png"))
        .or_else(|| IMAGES_DIR.get_file(format!("{image_name}.jpg")))
        .or_else(|| IMAGES_DIR.get_file(format!("{image_name}.gif")))
        .unwrap_or_else(|| panic!("image not found: {image_name}"));

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

    let mut distance_psd_cases = psd_all_cases
        .par_iter()
        .map(move |(case, psd_image)| {
            let psd_image_dssim = context
                .create_image(&imgref::ImgVec::new(
                    psd_image.to_luma32f().into_raw(),
                    width as usize,
                    height as usize,
                ))
                .unwrap();

            let (distance, _) = context.compare(&image, psd_image_dssim);

            (f64::from(distance), case)
        })
        .collect::<Vec<_>>();
    distance_psd_cases.sort_by(|(a_distance, _), (b_distance, _)| a_distance.total_cmp(b_distance));

    for (index, (_, case)) in distance_psd_cases.iter().enumerate().rev() {
        println!(
            "{index}. output/{case_id}.png {case_id}",
            case_id = case.case_id
        );
    }
    panic!("done");
}

fn copy_used_assets(used_background_image_names: &Vec<String>, used_cg_file_names: &Vec<String>) {
    copy_assets(
        used_background_image_names,
        &PathBuf::from_str("src/images").unwrap(),
        &PathBuf::from_str("used-assets/images").unwrap(),
    );
    copy_assets(
        used_cg_file_names,
        &PathBuf::from_str("src/psds").unwrap(),
        &PathBuf::from_str("used-assets/psds").unwrap(),
    );
    let additional_image_dir = PathBuf::from_str("src/additional_graphic/images").unwrap();
    let additional_image_names = additional_image_dir
        .read_dir()
        .unwrap()
        .filter_map(|dirent| {
            let dirent = dirent.unwrap();
            if !dirent.file_type().unwrap().is_file() {
                return None;
            }
            Some(
                PathBuf::from_str(dirent.file_name().to_str().unwrap())
                    .unwrap()
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
            )
        })
        .collect::<Vec<_>>();
    copy_assets(
        &additional_image_names,
        &additional_image_dir,
        &PathBuf::from_str("used-assets/additional_images").unwrap(),
    );
}

fn copy_assets(asset_names: &Vec<String>, source_dir: &PathBuf, dest_dir_path: &PathBuf) {
    if let Err(error) = fs::remove_dir_all(dest_dir_path) {
        if error.kind() != ErrorKind::NotFound {
            panic!("{:?}", error);
        }
    };
    fs::create_dir_all(dest_dir_path).unwrap();
    let files = HashMap::<String, DirEntry>::from_iter(
        source_dir
            .read_dir()
            .unwrap()
            .map(|dirent| dirent.unwrap())
            .filter(|dirent| dirent.file_type().unwrap().is_file())
            .map(|file| {
                (
                    file.path()
                        .file_stem()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                    file,
                )
            }),
    );
    for asset_name in asset_names {
        let file = files
            .get(asset_name)
            .unwrap_or_else(|| panic!("Asset not found: {asset_name:}"));
        fs::copy(
            source_dir.join(file.path().file_name().unwrap()),
            dest_dir_path.join(file.path().file_name().unwrap()),
        )
        .unwrap();
    }
}

fn get_image_id(bytes: &[u8]) -> Uuid {
    
    {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(bytes);
        let hash = hasher.finalize().to_le_bytes();
        let bytes: [u8; 16] = [
            hash[0], hash[1], hash[2], hash[3], hash[0], hash[1], hash[2], hash[3], hash[0],
            hash[1], hash[2], hash[3], hash[0], hash[1], hash[2], hash[3],
        ];
        Uuid::from_bytes(bytes)
    }
}

fn get_distance_psd_image_triples<'psd>(
    input: &Input,
    psd_all_cases: &'psd Vec<(PsdCase, image::DynamicImage)>,
) -> Vec<(f64, &'psd PsdCase, String, Uuid)> {
    if CHECKPOINT == 1 {
        let _result = {
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
                        .or_else(|| IMAGES_DIR.get_file(format!("{image_name}.jpg")))
                        .or_else(|| IMAGES_DIR.get_file(format!("{image_name}.gif")))
                        .unwrap_or_else(|| panic!("image not found: {image_name}"));

                    // let image_hash =
                    //     get_image_hash(&format!("src/images/{}", image_path.to_str().unwrap()));

                    let image_id = get_image_id(static_image.contents());

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

                            let (distance, _) = context.compare(&image, psd_image_dssim);

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
                    (distance, psd_case, image_url, image_id)
                })
                .collect::<Vec<_>>();

            for (distance, PsdCase { case_id, .. }, image_url, _image_id) in &result {
                println!("{distance}] {case_id} <-> {image_url}");
            }

            let distance_psd_image_triples_json = serde_json::to_string_pretty(
                &result
                    .iter()
                    .map(|(distance, psd_case, image_url, image_id)| {
                        (distance, psd_case.case_id, image_url, image_id)
                    })
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            fs::write(
                "src/distance_psd_image_triples.json",
                distance_psd_image_triples_json,
            )
            .unwrap();

            result
        };
        panic!("done");
    }

    let result = {
        let distance_psd_image_triples_json: Vec<(f64, Uuid, String, Uuid)> =
            serde_json::from_str(include_str!("distance_psd_image_triples.json")).unwrap();

        let psd_all_cases: HashMap<Uuid, &'psd PsdCase> = HashMap::from_iter(
            psd_all_cases
                .iter()
                .map(|(psd_case, _image)| (psd_case.case_id, psd_case)),
        );

        distance_psd_image_triples_json
            .into_iter()
            .map(|(distance, psd_case_id, image_url, image_id)| {
                (
                    distance,
                    *psd_all_cases.get(&psd_case_id).unwrap(),
                    image_url,
                    image_id,
                )
            })
            .collect::<Vec<_>>()
    };

    result
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
    if CHECKPOINT == 0 {
        let psds = PSDS_DIR
            .files()
            .par_bridge()
            .map(|psd_file| {
                server_core::apis::cg::shared::psd_to_cg_file::psd_to_webps_and_cg_file(
                    psd_file.contents(),
                    psd_file
                        .path()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .trim_end_matches(".psd"),
                )
                .unwrap_or_else(|_| panic!("failed to parse psd: {:?}", psd_file.path()))
            })
            .collect::<Vec<_>>();

        for psd in psds.iter() {
            if psd.variants_webps.is_empty() {
                panic!("psd has no variants: {:?}", psd.cg_file);
            }
        }

        fs::create_dir_all("output")?;

        let psd_all_cases: Vec<PsdCase> = psds
            .into_par_iter()
            .flat_map(
                |PsdParsingResult {
                    variants_webps,
                    cg_file,
                    cg_thumbnail_webp,
                 }| {
                    fn generate_all_cases(parts: Vec<CgPart>) -> Vec<Vec<ScreenCgPart>> {
                        if parts.is_empty() {
                            return vec![];
                        }
                        let (first_part, rest_parts) = parts.split_first().unwrap();
                        let variant_cases = variant_cases(first_part.clone());
                        let rest_parts_cases = generate_all_cases(rest_parts.to_vec());
                        if rest_parts_cases.is_empty() {
                            return variant_cases.into_iter().map(|x| vec![x]).collect();
                        }

                        variant_cases
                            .into_iter()
                            .flat_map(|variant| {
                                rest_parts_cases.clone().into_iter().map(
                                    move |mut rest_parts_case| {
                                        rest_parts_case.insert(0, variant.clone());
                                        rest_parts_case
                                    },
                                )
                            })
                            .collect()
                    }

                    let variant_image_buffers = variants_webps.into_par_iter().map(|(variant_id, webp)| {
                        let image_buffer = image::load_from_memory_with_format(
                            &webp,
                            image::ImageFormat::WebP,
                        ).expect("failed to load webp").to_rgba8();
                        (variant_id, image_buffer)
                    }).collect::<Vec<_>>();
                    
                    let cg_wh = {
                        let cg_thumbnail_image_buffer = image::load_from_memory_with_format(&cg_thumbnail_webp, image::ImageFormat::WebP).expect("failed to load webp");
                        Wh::new(cg_thumbnail_image_buffer.width(), cg_thumbnail_image_buffer.height())
                    };

                    let all_cases = generate_all_cases(cg_file.parts.clone());
                    all_cases.into_par_iter().map(move |parts| {
                        let cg_file = cg_file.clone();

                        let variants = parts.iter().flat_map(|screen_cg_part| {
                            cg_file.parts.iter().find(|cg_part| cg_part.name == screen_cg_part.name()).expect("cg_part not found")
                            .variants.iter()
                            .filter(move |cg_part_variant| screen_cg_part.is_variant_selected(&cg_part_variant.name))
                        }).collect::<Vec<_>>();

                        let mut bottom = RenderResult {
                            x: 0,
                            y: 0,
                            image_buffer: image::ImageBuffer::<image::Rgba<u8>, _>::new(cg_wh.width, cg_wh.height),
                        };
                        for variant in variants.iter().rev() {
                            let (_, image_buffer) = variant_image_buffers.iter().find(|(variant_id, _)| 
                                variant_id == &variant.id
                            ).expect("variant not found");
                            let src = RenderResult {
                                x: (cg_wh.width as f32 * variant.rect.x().as_f32()) as u32,
                                y: (cg_wh.height as f32 * variant.rect.y().as_f32()) as u32,
                                image_buffer: image_buffer.clone(),
                            };
                            bottom = server_core::apis::cg::shared::layer_tree::blend_buffer(&src, &bottom, match variant.blend_mode {
                                rpc::data::CgPartVariantBlendMode::PassThrough => psd::BlendMode::PassThrough,
                                rpc::data::CgPartVariantBlendMode::Normal => psd::BlendMode::Normal,
                                rpc::data::CgPartVariantBlendMode::Dissolve => psd::BlendMode::Dissolve,
                                rpc::data::CgPartVariantBlendMode::Darken => psd::BlendMode::Darken,
                                rpc::data::CgPartVariantBlendMode::Multiply => psd::BlendMode::Multiply,
                                rpc::data::CgPartVariantBlendMode::ColorBurn => psd::BlendMode::ColorBurn,
                                rpc::data::CgPartVariantBlendMode::LinearBurn => psd::BlendMode::LinearBurn,
                                rpc::data::CgPartVariantBlendMode::DarkerColor => psd::BlendMode::DarkerColor,
                                rpc::data::CgPartVariantBlendMode::Lighten => psd::BlendMode::Lighten,
                                rpc::data::CgPartVariantBlendMode::Screen => psd::BlendMode::Screen,
                                rpc::data::CgPartVariantBlendMode::ColorDodge => psd::BlendMode::ColorDodge,
                                rpc::data::CgPartVariantBlendMode::LinearDodge => psd::BlendMode::LinearDodge,
                                rpc::data::CgPartVariantBlendMode::LighterColor => psd::BlendMode::LighterColor,
                                rpc::data::CgPartVariantBlendMode::Overlay => psd::BlendMode::Overlay,
                                rpc::data::CgPartVariantBlendMode::SoftLight => psd::BlendMode::SoftLight,
                                rpc::data::CgPartVariantBlendMode::HardLight => psd::BlendMode::HardLight,
                                rpc::data::CgPartVariantBlendMode::VividLight => psd::BlendMode::VividLight,
                                rpc::data::CgPartVariantBlendMode::LinearLight => psd::BlendMode::LinearLight,
                                rpc::data::CgPartVariantBlendMode::PinLight => psd::BlendMode::PinLight,
                                rpc::data::CgPartVariantBlendMode::HardMix => psd::BlendMode::HardMix,
                                rpc::data::CgPartVariantBlendMode::Difference => psd::BlendMode::Difference,
                                rpc::data::CgPartVariantBlendMode::Exclusion => psd::BlendMode::Exclusion,
                                rpc::data::CgPartVariantBlendMode::Subtract => psd::BlendMode::Subtract,
                                rpc::data::CgPartVariantBlendMode::Divide => psd::BlendMode::Divide,
                                rpc::data::CgPartVariantBlendMode::Hue => psd::BlendMode::Hue,
                                rpc::data::CgPartVariantBlendMode::Saturation => psd::BlendMode::Saturation,
                                rpc::data::CgPartVariantBlendMode::Color => psd::BlendMode::Color,
                                rpc::data::CgPartVariantBlendMode::Luminosity => psd::BlendMode::Luminosity,
                            })
                        }

                        let case_id = {
                            let variants = parts
                                .iter()
                                .flat_map(|part| {
                                    let variants_in_part = cg_file
                                        .parts
                                        .iter()
                                        .filter(|x| x.name == part.name())
                                        .flat_map(|part| part.variants.clone())
                                        .collect::<Vec<_>>();

                                    match part {
                                        ScreenCgPart::Single { variant_name, .. } => {
                                            match variant_name {
                                                Some(variant_name) => {
                                                    vec![variants_in_part
                                                        .iter()
                                                        .find(|x| x.name == *variant_name)
                                                        .unwrap_or_else(|| panic!("Variant {} not found in {} / {}\n{:#?}",
                                                        variant_name,
                                                        cg_file.name,
                                                        part.name(),
                                                        cg_file)).clone()]
                                                }
                                                None => vec![],
                                            }
                                        }
                                        ScreenCgPart::Multi { variant_names, .. } => variants_in_part
                                            .into_iter()
                                            .filter(|variant| variant_names.contains(&variant.name))
                                            .collect(),
                                        ScreenCgPart::AlwaysOn { .. } => variants_in_part.clone(),
                                    }
                                })
                                .collect::<Vec<_>>();

                            namui_type::uuid_from_hash(
                                variants
                                    .iter()
                                    .map(|variant| variant.id)
                                    .collect::<Vec<_>>(),
                            )
                        };

                        let file_path = format!("output/{case_id}.png");
                        bottom.image_buffer.save(&file_path).unwrap();

                        // let image_hash = get_image_hash(&file_path);
                        let image_hash = 0;

                        PsdCase {
                            cg_file,
                            image_hash,
                            parts,
                            case_id,
                        }
                    })
                },
            )
            .collect::<Vec<_>>();

        println!("psd_all_cases: {:?}", psd_all_cases.len());
        let data = psd_all_cases
            .iter()
            .map(|case| (case.case_id, case.clone()))
            .collect::<BTreeMap<_, _>>();

        let case_metadata = CaseMetadata { data };
        fs::write(
            "output/case_metadata.json",
            serde_json::to_string_pretty(&case_metadata)?,
        )?;

        panic!("done");
    }

    let metadata: CaseMetadata =
        serde_json::from_str(&fs::read_to_string("output/case_metadata.json")?)?;

    let psd_all_cases = metadata
        .data
        .into_par_iter()
        .map(|(uuid, case)| {
            if CHECKPOINT == 1 {
                let buffer = image::open(format!("output/{uuid}.png"))
                    .unwrap()
                    .resize_to_fill(256, 256, FilterType::Nearest);
                return (case, buffer.to_luma32f().into());
            }
            (
                case,
                ImageBuffer::<Luma<f32>, Vec<f32>>::new(256, 256).into(),
            )
        })
        .collect::<Vec<_>>();

    Ok(psd_all_cases)
}

// fn find_psd_for_image(raw_image_buffer: &[u8], psds: &[PsdParsingResult]) -> Result<()> {
//     let image = image::load_from_memory(raw_image_buffer)?;
//     // todo parse image
//     // todo check size
//     psds.into_par_iter().find_any(|psd| {
//         // let psd_pixel_count = {
//         //     let first_layer = &psd.variants_bitmaps.first().unwrap().1;
//         //     first_layer.len() / 4
//         // };
//         // let image_pixel_count = image.width() * image.height();
//         // if psd_pixel_count != image_pixel_count as usize {
//         //     println!("psd_pixel_count: {psd_pixel_count}, image_pixel_count: {image_pixel_count}");
//         //     return false;
//         // }

//         fn generate_all_cases(parts: Vec<CgPart>) -> Vec<Vec<CgPartVariant>> {
//             if parts.len() == 0 {
//                 return vec![];
//             }
//             let (first_part, rest_parts) = parts.split_first().unwrap();
//             let variant_cases = variant_cases(first_part.clone());
//             let rest_parts_cases = generate_all_cases(rest_parts.to_vec());

//             variant_cases
//                 .into_iter()
//                 .flat_map(|variant| {
//                     rest_parts_cases
//                         .clone()
//                         .into_iter()
//                         .map(move |mut rest_parts_case| {
//                             rest_parts_case.insert(0, variant.clone());
//                             rest_parts_case
//                         })
//                 })
//                 .collect::<Vec<_>>()
//         }

//         let all_cases = generate_all_cases(psd.cg_file.parts.clone());

//         todo!()
//     });

//     Ok(())
// }

fn variant_cases(part: CgPart) -> Vec<ScreenCgPart> {
    match part.selection_type {
        rpc::data::PartSelectionType::Single => {
            let name = part.name;
            let mut cases = vec![];
            cases.push(ScreenCgPart::Single {
                name: name.clone(),
                variant_name: None,
            });
            for variant in part.variants {
                let variant_name = Some(variant.name);
                cases.push(ScreenCgPart::Single {
                    name: name.clone(),
                    variant_name,
                })
            }
            cases
        }
        rpc::data::PartSelectionType::Multi => {
            let name = part.name;
            let mut cases = vec![];
            cases.push(ScreenCgPart::Multi {
                name: name.clone(),
                variant_names: HashSet::new(),
            });
            for variant in part.variants {
                let variant_names = HashSet::from_iter(std::iter::once(variant.name));
                cases.push(ScreenCgPart::Multi {
                    name: name.clone(),
                    variant_names,
                });
            }
            cases
        }
        rpc::data::PartSelectionType::AlwaysOn => {
            let name = part.name;
            vec![ScreenCgPart::AlwaysOn { name }]
        }
    }
}

fn handle_images<'psd>(
    cut: &mut Cut,
    images: Vec<Image>,
    image_url_psd_case_map: &HashMap<String, (f64, &'psd PsdCase, Uuid)>,
    predetermined_graphic_map: &BTreeMap<Uuid, PredeterminedGraphic<'psd>>,
    used_background_images: &mut HashMap<Uuid, String>,
    used_cg_file_names: &mut BTreeSet<String>,
) {
    let mut character_images = vec![];
    let mut background_images = vec![];

    fn insert_image<'psd>(
        used_background_images: &mut HashMap<Uuid, String>,
        background_images: &mut Vec<(Image, &'psd PsdCase, Uuid)>,
        image: Image,
        psd_case: &'psd PsdCase,
        image_id: Uuid,
    ) {
        used_background_images.insert(image_id, image.url.split('/').last().unwrap().to_string());
        background_images.insert(0, (image, psd_case, image_id));
    }
    fn insert_cg<'psd>(
        used_cg_file_names: &mut BTreeSet<String>,
        character_images: &mut Vec<(Image, &'psd PsdCase, Uuid)>,
        image: Image,
        psd_case: &'psd PsdCase,
        image_id: Uuid,
    ) {
        used_cg_file_names.insert(psd_case.cg_file.name.clone());
        character_images.insert(0, (image, psd_case, image_id));
    }

    for image in images {
        let (distance, psd_case, image_id) = image_url_psd_case_map.get(&image.url).unwrap();

        let Some(predetermined_graphic) = predetermined_graphic_map.get(image_id) else {
            if *distance >= BACKGROUND_IMAGE_DISTANCE_THRESHOLD {
                insert_image(
                    used_background_images,
                    &mut background_images,
                    image,
                    psd_case,
                    *image_id,
                );
            } else {
                insert_cg(
                    used_cg_file_names,
                    &mut character_images,
                    image,
                    psd_case,
                    *image_id,
                );
            }
            continue;
        };

        match predetermined_graphic {
            PredeterminedGraphic::PlainImage => insert_image(
                used_background_images,
                &mut background_images,
                image,
                psd_case,
                *image_id,
            ),
            PredeterminedGraphic::Cg { psd_case } => insert_cg(
                used_cg_file_names,
                &mut character_images,
                image,
                psd_case,
                *image_id,
            ),
        }
    }

    const ASPECT_RATIO: f64 = 4.0 / 3.0;
    let backgrounds = background_images.into_iter().map(|(image, _, image_id)| {
        (
            Uuid::new_v4(),
            ScreenGraphic::Image(ScreenImage {
                id: image_id,
                circumscribed: percent_xywh_to_circumscribed(image.xywh, ASPECT_RATIO),
                rotation: Angle::Degree(0.0)
            }),
        )
    });
    let characters = character_images.into_iter().map(|(image, psd_case, _)| {
        (
            Uuid::new_v4(),
            ScreenGraphic::Cg(ScreenCg {
                id: psd_case.cg_file.id,
                name: psd_case.cg_file.name.clone(),
                parts: psd_case.parts.clone(),
                circumscribed: percent_xywh_to_circumscribed(image.xywh, ASPECT_RATIO),
                rotation: Angle::Degree(0.0)
            }),
        )
    });

    cut.screen_graphics = characters.chain(backgrounds).collect();
}

fn handle_texts(cut: &mut Cut, texts: Vec<Text>) {
    if texts.is_empty() {
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

/// aspect_ratio = width / height;
///
/// ex) 4:3 should be 1.333...
fn percent_xywh_to_circumscribed(xywh: Xywh, aspect_ratio: f64) -> Circumscribed<Percent> {
    let center_xy = Xy {
        x: percent(((xywh.x + xywh.width / 2.0) * 100.0) as f32),
        y: percent(((xywh.y + xywh.height / 2.0) * 100.0) as f32),
    };
    let radius = percent(
        ((((xywh.width * aspect_ratio).powi(2) + (xywh.height).powi(2))
            / (1.0 + aspect_ratio.powi(2)))
        .sqrt()
            * 100.0) as f32,
    );
    Circumscribed { center_xy, radius }
}
