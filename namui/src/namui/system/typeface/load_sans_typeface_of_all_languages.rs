use crate::*;
use futures::{future::try_join_all, try_join};
use std::{collections::HashMap, sync::Arc};

const DEFAULT_FONT_SIZE: IntPx = int_px(12);

type TypefaceFileUrls = HashMap<TypefaceType, String>;
type TypefaceFileUrlsFile = HashMap<Language, HashMap<FontWeight, String>>;

pub async fn load_all_typefaces() -> Result<(), Box<dyn std::error::Error>> {
    try_join!(
        load_fallback_font_typefaces(),
        load_sans_typeface_of_all_languages()
    )?;

    Ok(())
}

async fn load_fallback_font_typefaces() -> Result<(), Box<dyn std::error::Error>> {
    let typeface = Arc::new(get_noto_color_emoji_typeface().await?);
    crate::typeface::load_fallback_font_typeface("emoji".to_string(), typeface.clone());
    load_default_font_of_typeface(typeface);

    Ok(())
}

async fn get_noto_color_emoji_typeface() -> Result<Typeface, Box<dyn std::error::Error>> {
    let url = crate::Url::parse("bundle:__system__/font/NotoColorEmoji.woff2")?;
    get_typeface(url).await
}

pub async fn load_sans_typeface_of_all_languages() -> Result<(), Box<dyn std::error::Error>> {
    let typeface_file_urls: TypefaceFileUrls = get_typeface_file_urls().await?;

    let typefaces = get_typefaces_from_file_urls(typeface_file_urls).await?;
    typefaces.into_iter().for_each(|(typeface_type, typeface)| {
        crate::typeface::load_typeface(&typeface_type, typeface.clone());
        load_default_font_of_typeface(typeface);
    });

    Ok(())
}

async fn get_typefaces_from_file_urls(
    typeface_file_urls: TypefaceFileUrls,
) -> Result<HashMap<TypefaceType, Arc<Typeface>>, Box<dyn std::error::Error>> {
    let iter = try_join_all(typeface_file_urls.iter().map(
        |(typeface_type, font_file_url)| async move {
            let url = crate::Url::parse(font_file_url)?;
            get_typeface(url)
                .await
                .map(|typeface| (*typeface_type, Arc::new(typeface)))
        },
    ))
    .await?;
    Ok(HashMap::from_iter(iter))
}

async fn get_typeface(url: Url) -> Result<Typeface, Box<dyn std::error::Error>> {
    let typeface = match crate::cache::get(url.as_str()).await? {
        Some(cached_bytes) => Typeface::new(&cached_bytes),
        None => {
            let bytes = crate::file::bundle::read(&url)
                .await
                .expect(format!("Could not fetch {}", url).as_str());
            crate::cache::set(url.as_str(), bytes.as_ref()).await?;
            Typeface::new(&bytes)
        }
    };
    Ok(typeface)
}

async fn load_typeface_file_urls_file() -> Result<TypefaceFileUrlsFile, Box<dyn std::error::Error>>
{
    let url = crate::Url::parse("bundle:__system__/font/map.json")?;
    let Some(cached_typeface_file_urls_file) = crate::cache::get_serde(url.as_str()).await? else {
        let typeface_file_urls_file = crate::file::bundle::read_json(url.clone()).await?;
        crate::cache::set_serde(url.as_str(), &typeface_file_urls_file).await?;
        return Ok(typeface_file_urls_file)
    };
    Ok(cached_typeface_file_urls_file)
}

async fn get_typeface_file_urls() -> Result<TypefaceFileUrls, Box<dyn std::error::Error>> {
    let typeface_file_map_file: TypefaceFileUrlsFile = load_typeface_file_urls_file().await?;

    Ok(typeface_file_map_file
        .iter()
        .flat_map(|(language, font_file_map)| {
            font_file_map
                .iter()
                .map(move |(font_weight, font_file_url)| {
                    (
                        TypefaceType {
                            serif: false,
                            font_weight: font_weight.clone(),
                            language: *language,
                        },
                        font_file_url.clone(),
                    )
                })
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn font_file_url_map_should_be_serializable() {
        let font_file_url_map: TypefaceFileUrlsFile = serde_json::from_str(
            r#"
        {
            "Ko": {
                "100": "ko/NotoSansKR-Thin.woff2",
                "300": "ko/NotoSansKR-Light.woff2",
                "400": "ko/NotoSansKR-Regular.woff2",
                "500": "ko/NotoSansKR-Medium.woff2",
                "700": "ko/NotoSansKR-Bold.woff2",
                "900": "ko/NotoSansKR-Black.woff2"
            }
        }"#,
        )
        .unwrap();

        let answer = HashMap::from([(
            Language::Ko,
            HashMap::from([
                (FontWeight::_100, "ko/NotoSansKR-Thin.woff2".to_string()),
                (FontWeight::_300, "ko/NotoSansKR-Light.woff2".to_string()),
                (FontWeight::_400, "ko/NotoSansKR-Regular.woff2".to_string()),
                (FontWeight::_500, "ko/NotoSansKR-Medium.woff2".to_string()),
                (FontWeight::_700, "ko/NotoSansKR-Bold.woff2".to_string()),
                (FontWeight::_900, "ko/NotoSansKR-Black.woff2".to_string()),
            ]),
        )]);

        assert_eq!(font_file_url_map, answer);

        let serialized_font_file_url_map = serde_json::to_string(&font_file_url_map).unwrap();
        // NOTE: We don't test `serialized_font_file_url_map` because it's hashmap, order is random.

        let deserialized_font_file_url_map: TypefaceFileUrlsFile =
            serde_json::from_str(&serialized_font_file_url_map).unwrap();
        assert_eq!(deserialized_font_file_url_map, answer);
    }
}

fn load_default_font_of_typeface(typeface: Arc<Typeface>) {
    font::get_font_of_typeface(typeface, DEFAULT_FONT_SIZE);
}
