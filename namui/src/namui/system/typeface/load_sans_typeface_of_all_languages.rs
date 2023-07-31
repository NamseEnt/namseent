use crate::*;
use ::futures::{future::try_join_all, try_join};
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
    let bytes = get_file_from_bundle_with_cached(&url)
        .await
        .expect(format!("Could not fetch {}", url).as_str());

    Ok(Typeface::new(&bytes))
}

pub async fn load_sans_typeface_of_all_languages() -> Result<(), Box<dyn std::error::Error>> {
    let typeface_file_urls: TypefaceFileUrls = get_typeface_file_urls().await?;

    let typeface_files = get_typeface_files(&typeface_file_urls).await?;
    typeface_files.iter().for_each(|(typeface_type, bytes)| {
        let typeface = Arc::new(Typeface::new(bytes));
        crate::typeface::load_typeface(&typeface_type, typeface.clone());
        load_default_font_of_typeface(typeface);
    });

    Ok(())
}

async fn load_typeface_file_urls_file() -> Result<TypefaceFileUrlsFile, Box<dyn std::error::Error>>
{
    let url = crate::Url::parse("bundle:__system__/font/map.json")?;
    match serde_json::from_slice(&get_file_from_bundle_with_cached(&url).await?) {
        Ok(typeface_file_urls_file) => Ok(typeface_file_urls_file),
        Err(error) => Err(format!("{error:?}").into()),
    }
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

async fn get_typeface_files(
    typeface_file_urls: &TypefaceFileUrls,
) -> Result<HashMap<TypefaceType, Vec<u8>>, Box<dyn std::error::Error>> {
    let iter = try_join_all(typeface_file_urls.iter().map(
        |(typeface_type, font_file_url)| async move {
            let url = crate::Url::parse(font_file_url)?;
            let result: Result<_, Box<dyn std::error::Error>> =
                match get_file_from_bundle_with_cached(&url).await {
                    Ok(bytes) => Ok((*typeface_type, bytes)),
                    Err(error) => Err(format!("Could not fetch {font_file_url} - {error}").into()),
                };
            result
        },
    ))
    .await?;

    Ok(HashMap::from_iter(iter))
}

async fn get_file_from_bundle_with_cached(
    url: &crate::Url,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let file = match crate::cache::get(url.as_str()).await? {
        Some(cached_file) => cached_file.to_vec(),
        None => {
            let file = crate::file::bundle::read(url.clone())
                .await?
                .as_ref()
                .to_vec();
            crate::cache::set(url.as_str(), &file).await?;
            file
        }
    };
    Ok(file)
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
