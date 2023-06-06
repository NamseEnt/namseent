mod psd_parsing;

use anyhow::Result;
use include_dir::{include_dir, Dir};
use namui_type::{percent, Percent, Uuid, Xy};
// use opencv::prelude::*;
use image::{imageops::FilterType, DynamicImage, ImageBuffer, Luma};
use psd_parsing::{parse_psd, PsdParsingResult};
use rayon::prelude::*;
use rpc::{
    data::{
        CgFile, CgPart, CgPartVariant, Circumscribed, Cut, ScreenCg, ScreenGraphic, ScreenImage,
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
/// 2. Generate plain image ids and save them to `src/definite_plain_image_ids.json`.
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

    // list_similar_psd_cases(&psd_all_cases, "1");

    let distance_psd_image_triples = get_distance_psd_image_triples(&input, &psd_all_cases);

    let plain_image_id_set = get_plain_image_id_set();

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
            &plain_image_id_set,
            &mut used_background_images,
            &mut used_cg_file_names,
        );

        sequence.cuts.push(cut);
    }

    println!("Used background image urls, please upload them to project as image");
    let mut used_background_image_names = used_background_images
        .into_iter()
        .map(|(_, name)| name)
        .collect::<Vec<_>>();
    used_background_image_names.sort();
    for image_name in used_background_image_names.iter() {
        println!("{image_name}");
    }

    println!("");
    println!("Used cg file names, please upload them to project as cg file");
    let used_cg_file_names: Vec<_> = used_cg_file_names.into_iter().collect();
    for cg_file_name in used_cg_file_names.iter() {
        println!("{cg_file_name}");
    }

    let sequence_json = serde_json::to_string(&sequence).unwrap();

    fs::write("sequence.json", sequence_json).unwrap();

    copy_used_assets(&used_background_image_names, &used_cg_file_names);

    Ok(())
}

fn list_similar_psd_cases(psd_all_cases: &[(PsdCase, DynamicImage)], image_name: &str) {
    let static_image = IMAGES_DIR
        .get_file(&format!("{image_name}.png"))
        .or_else(|| IMAGES_DIR.get_file(&format!("{image_name}.jpg")))
        .or_else(|| IMAGES_DIR.get_file(&format!("{image_name}.gif")))
        .expect(&format!("image not found: {image_name}"));

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

            let (distance, _) = context.compare(&image, &psd_image_dssim);

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
            .expect(&format!("Asset not found: {asset_name:}"));
        fs::copy(
            source_dir.join(file.path().file_name().unwrap()),
            dest_dir_path.join(file.path().file_name().unwrap()),
        )
        .unwrap();
    }
}

fn get_plain_image_id_set() -> HashSet<Uuid> {
    if CHECKPOINT == 2 {
        let plain_image_names: Vec<String> =
            serde_json::from_str(include_str!("plain_image_names.json")).unwrap();

        let result =
            HashSet::<Uuid>::from_par_iter(plain_image_names.into_par_iter().map(|image_name| {
                let static_image = IMAGES_DIR
                    .get_file(&format!("{image_name}.png"))
                    .or_else(|| IMAGES_DIR.get_file(&format!("{image_name}.jpg")))
                    .or_else(|| IMAGES_DIR.get_file(&format!("{image_name}.gif")))
                    .expect(&format!("image not found: {image_name}"));

                let image_id = get_image_id(static_image.contents());
                image_id
            }));

        let plain_image_ids_json = serde_json::to_string(&result).unwrap();

        fs::write("src/plain_image_ids.json", plain_image_ids_json).unwrap();

        panic!("done");
    }

    let plain_image_ids: HashSet<Uuid> =
        serde_json::from_str(&fs::read_to_string("src/plain_image_ids.json").unwrap()).unwrap();

    plain_image_ids
}

fn get_image_id(bytes: &[u8]) -> Uuid {
    let image_id = {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(bytes);
        let hash = hasher.finalize().to_le_bytes();
        let bytes: [u8; 16] = [
            hash[0], hash[1], hash[2], hash[3], hash[0], hash[1], hash[2], hash[3], hash[0],
            hash[1], hash[2], hash[3], hash[0], hash[1], hash[2], hash[3],
        ];
        Uuid::from_bytes(bytes)
    };
    image_id
}

fn get_distance_psd_image_triples<'psd>(
    input: &Input,
    psd_all_cases: &'psd Vec<(PsdCase, image::DynamicImage)>,
) -> Vec<(f64, &'psd PsdCase, String, Uuid)> {
    if CHECKPOINT == 1 {
        let result = {
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
                .map(|(psd_case, _image)| (psd_case.case_id.clone(), psd_case)),
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
                parse_psd(
                    psd_file.contents(),
                    psd_file
                        .path()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .trim_end_matches(".psd"),
                )
                .expect(format!("failed to parse psd: {:?}", psd_file.path()).as_str())
            })
            .collect::<Vec<_>>();

        for psd in psds.iter() {
            if psd.variants_images.is_empty() {
                panic!("psd has no variants: {:?}", psd.cg_file);
            }
        }

        fs::create_dir_all("output")?;

        let psd_all_cases: Vec<(PsdCase, image::DynamicImage)> = psds
            .into_par_iter()
            .flat_map(
                |PsdParsingResult {
                     variants_images,
                     cg_file,
                     wh,
                 }| {
                    fn generate_all_cases(parts: Vec<CgPart>) -> Vec<Vec<CgPartVariant>> {
                        if parts.len() == 0 {
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
                            .collect::<Vec<_>>()
                    }

                    let all_cases = generate_all_cases(cg_file.parts.clone());

                    all_cases.into_par_iter().map(move |variants| {
                        let cg_file = cg_file.clone();

                        let layer_images = variants
                            .clone()
                            .into_par_iter()
                            .map(|variant| {
                                variants_images
                                    .iter()
                                    .find_map(|variants_image| {
                                        if variants_image.variant_id == variant.id {
                                            Some(&variants_image.image_buffer)
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap()
                            })
                            .collect::<Vec<_>>();

                        let mut bottom =
                            image::ImageBuffer::<image::Rgba<u8>, _>::new(wh.width, wh.height);

                        for part_image in layer_images.into_iter().rev() {
                            image::imageops::overlay(&mut bottom, part_image, 0, 0);
                        }

                        let case_id = namui_type::uuid_from_hash(
                            variants.iter().map(|x| x.id).collect::<Vec<_>>(),
                        );

                        let file_path = format!("output/{case_id}.png");
                        bottom.save(&file_path).unwrap();

                        // let image_hash = get_image_hash(&file_path);
                        let image_hash = 0;

                        (
                            PsdCase {
                                cg_file,
                                image_hash,
                                variants,
                                case_id,
                            },
                            bottom.into(),
                        )
                    })
                },
            )
            .collect::<Vec<_>>();

        println!("psd_all_cases: {:?}", psd_all_cases.len());
        let data = psd_all_cases
            .iter()
            .map(|(case, _image)| (case.case_id, case.clone()))
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
        rpc::data::PartSelectionType::Multi => part.variants,
        rpc::data::PartSelectionType::AlwaysOn => part.variants,
    }
}

fn handle_images(
    cut: &mut Cut,
    images: Vec<Image>,
    image_url_psd_case_map: &HashMap<String, (f64, &PsdCase, Uuid)>,
    plain_image_id_set: &HashSet<Uuid>,
    used_background_images: &mut HashMap<Uuid, String>,
    used_cg_file_names: &mut BTreeSet<String>,
) {
    let mut character_images = vec![];
    let mut background_images = vec![];

    for image in images {
        let (distance, psd_case, image_id) = image_url_psd_case_map.get(&image.url).unwrap();

        if plain_image_id_set.contains(image_id) || *distance >= BACKGROUND_IMAGE_DISTANCE_THRESHOLD
        {
            used_background_images
                .insert(*image_id, image.url.split('/').last().unwrap().to_string());
            background_images.push((image, psd_case, image_id));
        } else {
            used_cg_file_names.insert(psd_case.cg_file.name.clone());
            character_images.push((image, psd_case, image_id));
        }
    }

    const ASPECT_RATIO: f64 = 4.0 / 3.0;
    let backgrounds = background_images.into_iter().map(|(image, _, image_id)| {
        ScreenGraphic::Image(ScreenImage {
            id: *image_id,
            circumscribed: percent_xywh_to_circumscribed(image.xywh, ASPECT_RATIO),
        })
    });
    let characters = character_images.into_iter().map(|(image, psd_case, _)| {
        ScreenGraphic::Cg(ScreenCg {
            id: psd_case.cg_file.id,
            part_variants: psd_case
                .variants
                .iter()
                .map(|variant| (variant.id, variant.rect))
                .collect(),
            circumscribed: percent_xywh_to_circumscribed(image.xywh, ASPECT_RATIO),
        })
    });

    cut.screen_graphics = backgrounds.chain(characters).collect();
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
