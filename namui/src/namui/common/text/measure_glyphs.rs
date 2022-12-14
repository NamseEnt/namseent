use crate::*;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub enum GlyphMeasure {
    Failed,
    Success {
        glyph_id: u16,
        width: Px,
        font: Arc<Font>,
    },
}

pub(crate) fn measure_glyphs(
    text: &str,
    fonts: &Vec<Arc<Font>>,
    paint: Option<&Paint>,
) -> Vec<GlyphMeasure> {
    let cache_key = CacheKey {
        text: text.to_string(),
        fonts: fonts.to_vec(),
        paint_id: paint.map(|p| p.id),
    };
    if let Some(cached) = get_glyph_width_measures_cache(&cache_key) {
        return cached;
    }

    let mut not_calculated_chars: Vec<_> = text.chars().enumerate().collect();
    let mut measures: Vec<GlyphMeasure> = vec![GlyphMeasure::Failed; not_calculated_chars.len()];

    for font in fonts.iter() {
        if not_calculated_chars.is_empty() {
            break;
        }

        let not_calculated_text = not_calculated_chars
            .iter()
            .map(|(_, char)| char)
            .collect::<String>();

        let glyph_ids = font.get_glyph_ids(&not_calculated_text);

        if not_calculated_chars.len() != glyph_ids.len() {
            panic!(
                "non_calculated_chars.len(){} != glyph_ids.len(){}",
                not_calculated_chars.len(),
                glyph_ids.len()
            );
        }

        let non_zero_glyph_ids_with_char_index = glyph_ids
            .iter()
            .enumerate()
            .filter(|(_, glyph_id)| **glyph_id != 0)
            .map(|(index, glyph_id)| (not_calculated_chars[index].0, *glyph_id))
            .collect::<Vec<_>>();

        if non_zero_glyph_ids_with_char_index.is_empty() {
            continue;
        }

        let non_zero_glyph_char_indexes = non_zero_glyph_ids_with_char_index
            .iter()
            .map(|(index, _)| *index)
            .collect::<Vec<_>>();

        not_calculated_chars
            .retain(|(char_index, _)| !non_zero_glyph_char_indexes.contains(char_index));

        let glyph_id_char_index_and_width_vec: Vec<(u16, usize, Px)> = {
            let glyph_ids: Vec<u16> = non_zero_glyph_ids_with_char_index
                .iter()
                .map(|(_, glyph_id)| *glyph_id)
                .collect();

            let widths = font.get_glyph_widths(glyph_ids.into(), paint);

            widths
                .into_iter()
                .zip(non_zero_glyph_ids_with_char_index.into_iter())
                .map(|(width, (char_index, glyph_id))| (glyph_id, char_index, width))
                .collect()
        };

        for (glyph_id, char_index, width) in glyph_id_char_index_and_width_vec {
            measures[char_index] = GlyphMeasure::Success {
                width,
                font: font.clone(),
                glyph_id,
            };
        }
    }

    put_glyph_width_measures_cache(cache_key, measures.clone());

    measures
}

static GLYPH_WIDTH_MEASURES_CACHE: OnceCell<Mutex<lru::LruCache<CacheKey, Vec<GlyphMeasure>>>> =
    OnceCell::new();

struct CacheKey {
    text: String,
    fonts: Vec<Arc<Font>>,
    paint_id: Option<Uuid>,
}

impl std::hash::Hash for CacheKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.text.hash(state);
        self.fonts.hash(state);
        self.paint_id.hash(state);
    }
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.fonts == other.fonts && self.paint_id == other.paint_id
    }
}

impl Eq for CacheKey {}

fn get_glyph_width_measures_cache(key: &CacheKey) -> Option<Vec<GlyphMeasure>> {
    GLYPH_WIDTH_MEASURES_CACHE
        .get_or_init(|| Mutex::new(lru::LruCache::new(1024)))
        .lock()
        .unwrap()
        .get(key)
        .cloned()
}

fn put_glyph_width_measures_cache(key: CacheKey, value: Vec<GlyphMeasure>) {
    GLYPH_WIDTH_MEASURES_CACHE
        .get_or_init(|| Mutex::new(lru::LruCache::new(1024)))
        .lock()
        .unwrap()
        .put(key, value);
}
