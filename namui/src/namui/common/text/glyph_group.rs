use super::*;
use crate::*;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct GlyphGroup {
    pub glyph_ids: Vec<u16>,
    pub end_char_index: usize,
    pub width: Px,
    pub widths: Vec<Px>,
    pub font: Arc<Font>,
}

pub(crate) fn get_glyph_groups(
    text: &str,
    fonts: &Vec<Arc<Font>>,
    paint: Option<&Paint>,
) -> Vec<GlyphGroup> {
    let cache_key = CacheKey {
        text: text.to_string(),
        fonts: fonts.to_vec(),
        paint_id: paint.map(|p| p.id),
    };
    if let Some(cached) = get_glyph_groups_cache(&cache_key) {
        return cached;
    }

    let measures = measure_glyphs(text, fonts, paint);

    let mut groups: Vec<GlyphGroup> = vec![];

    let mut is_continue_to_last = false;
    for (char_index, measure) in measures.into_iter().enumerate() {
        match measure {
            GlyphMeasure::Failed => {
                is_continue_to_last = false;
            }
            GlyphMeasure::Success {
                glyph_id,
                width,
                font,
            } => {
                if is_continue_to_last {
                    if let Some(last_group) = groups.last_mut() {
                        if last_group.end_char_index + 1 == char_index && last_group.font == font {
                            last_group.glyph_ids.push(glyph_id);
                            last_group.width += width;
                            last_group.end_char_index = char_index;
                            last_group.widths.push(width);
                            continue;
                        }
                    }
                }

                groups.push(GlyphGroup {
                    glyph_ids: vec![glyph_id],
                    end_char_index: char_index,
                    width,
                    font: font.clone(),
                    widths: vec![width],
                });
                is_continue_to_last = true;
            }
        }
    }

    put_glyph_groups_cache(cache_key, groups.clone());

    groups
}
impl GlyphGroup {
    pub(crate) fn start_index(&self) -> usize {
        self.end_char_index + 1 - self.glyph_ids.len()
    }
}

static GLYPH_GROUPS_CACHE: OnceCell<Mutex<lru::LruCache<CacheKey, Vec<GlyphGroup>>>> =
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

fn get_glyph_groups_cache(key: &CacheKey) -> Option<Vec<GlyphGroup>> {
    GLYPH_GROUPS_CACHE
        .get_or_init(|| Mutex::new(lru::LruCache::new(1024)))
        .lock()
        .unwrap()
        .get(key)
        .cloned()
}

fn put_glyph_groups_cache(key: CacheKey, value: Vec<GlyphGroup>) {
    GLYPH_GROUPS_CACHE
        .get_or_init(|| Mutex::new(lru::LruCache::new(1024)))
        .lock()
        .unwrap()
        .put(key, value);
}
