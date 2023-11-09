use crate::*;
use namui_type::Uuid;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub(crate) enum PredeterminedGraphic<'psd> {
    PlainImage,
    Cg { psd_case: &'psd PsdCase },
}

type ImageId = Uuid;
type PredeterminedGraphicMap<'psd> = BTreeMap<ImageId, PredeterminedGraphic<'psd>>;

const PLAIN_IMAGE_NAMES_JSON_PATH: &str = "src/predetermined_graphic/plain_image_names.json";
const IMAGE_NAME_CG_CASE_ID_MAP_JSON_PATH: &str =
    "src/predetermined_graphic/image_name_cg_case_id_map.json";
const PREDETERMINED_GRAPHIC_MAP_JSON_PATH: &str =
    "src/predetermined_graphic/predetermined_graphic_map.json";

#[derive(Serialize, Deserialize)]
enum PredeterminedGraphicOnlyId {
    PlainImage,
    Cg { case_id: Uuid },
}

pub(crate) fn get_predetermined_graphic_map(
    psd_all_cases: &Vec<(PsdCase, DynamicImage)>,
) -> PredeterminedGraphicMap {
    if CHECKPOINT == 2 {
        generate_predetermined_graphic_map();
        panic!("done");
    }

    read_predetermined_graphic_map_cached(psd_all_cases)
}

fn generate_predetermined_graphic_map() {
    let plain_image_names: Vec<String> =
        serde_json::from_str(&fs::read_to_string(PLAIN_IMAGE_NAMES_JSON_PATH).unwrap()).unwrap();
    let image_name_cg_case_id_map: HashMap<String, Uuid> =
        serde_json::from_str(&fs::read_to_string(IMAGE_NAME_CG_CASE_ID_MAP_JSON_PATH).unwrap())
            .unwrap();

    let predetermined_graphic_map_only_id =
        BTreeMap::<Uuid, PredeterminedGraphicOnlyId>::from_par_iter(
            plain_image_names
                .into_par_iter()
                .map(|image_name| {
                    let image_id = read_image_as_image_id(&image_name);
                    (image_id, PredeterminedGraphicOnlyId::PlainImage)
                })
                .chain(
                    image_name_cg_case_id_map
                        .into_par_iter()
                        .map(|(image_name, case_id)| {
                            let image_id = read_image_as_image_id(&image_name);
                            (image_id, PredeterminedGraphicOnlyId::Cg { case_id })
                        }),
                ),
        );

    let predetermined_graphic_map_only_id_json_string =
        serde_json::to_string(&predetermined_graphic_map_only_id).unwrap();

    fs::write(
        PREDETERMINED_GRAPHIC_MAP_JSON_PATH,
        predetermined_graphic_map_only_id_json_string,
    )
    .unwrap();
}

fn read_image_as_image_id(image_name: &str) -> Uuid {
    let static_image = IMAGES_DIR
        .get_file(&format!("{image_name}.png"))
        .or_else(|| IMAGES_DIR.get_file(format!("{image_name}.jpg")))
        .or_else(|| IMAGES_DIR.get_file(format!("{image_name}.gif")))
        .unwrap_or_else(|| panic!("image not found: {image_name}"));

    let image_id = get_image_id(static_image.contents());
    image_id
}

fn read_predetermined_graphic_map_cached(
    psd_all_cases: &Vec<(PsdCase, DynamicImage)>,
) -> PredeterminedGraphicMap<'_> {
    let predetermined_graphic_map_only_id: BTreeMap<Uuid, PredeterminedGraphicOnlyId> =
        serde_json::from_slice(&fs::read(PREDETERMINED_GRAPHIC_MAP_JSON_PATH).unwrap()).unwrap();
    predetermined_graphic_map_only_id
        .into_iter()
        .map(
            |(image_id, predetermined_graphic_only_id)| match predetermined_graphic_only_id {
                PredeterminedGraphicOnlyId::PlainImage => {
                    (image_id, PredeterminedGraphic::PlainImage)
                }
                PredeterminedGraphicOnlyId::Cg { case_id } => (
                    image_id,
                    PredeterminedGraphic::Cg {
                        psd_case: psd_all_cases
                            .iter()
                            .find_map(|(psd_case, _)| match psd_case.case_id == case_id {
                                true => Some(psd_case),
                                false => None,
                            })
                            .unwrap(),
                    },
                ),
            },
        )
        .collect()
}
