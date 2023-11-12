use crate::*;
use std::collections::BTreeMap;

type CutIndexGraphicMap = BTreeMap<usize, Vec<ScreenGraphic>>;
const CUT_INDEX_GRAPHIC_MAP_JSON_PATH: &str = "src/additional_graphic/cut_index_graphic_map.json";

pub(crate) fn push_additional_graphic_map(sequence: &mut Sequence) {
    let cut_index_graphic_map: CutIndexGraphicMap =
        serde_json::from_str(&fs::read_to_string(CUT_INDEX_GRAPHIC_MAP_JSON_PATH).unwrap())
            .unwrap();

    for (cut_index, graphics) in cut_index_graphic_map.into_iter() {
        let cut = sequence.cuts.get_mut(cut_index).unwrap();
        for graphic in graphics {
            cut.screen_graphics.insert(0, (uuid(), graphic))
        }
    }
}
