use crate::{
    namui::{FontWeight, Language, TypefaceType},
    network::*,
    Typeface,
};
use futures::{future::join_all, try_join};
use std::{collections::HashMap, iter::FromIterator};

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
    let typeface = get_noto_color_emoji_typeface().await?;

    crate::typeface::load_fallback_font_typeface(typeface);
    Ok(())
}

async fn get_noto_color_emoji_typeface() -> Result<Typeface, Box<dyn std::error::Error>> {
    let url = "resources/font/NotoColorEmoji.woff2";
    let bytes = fetch_get_vec_u8(url)
        .await
        .expect(format!("Could not fetch {}", url).as_str());

    Ok(Typeface::new(&bytes))
}

pub async fn load_sans_typeface_of_all_languages() -> Result<(), Box<dyn std::error::Error>> {
    let typeface_file_urls: TypefaceFileUrls = get_typeface_file_urls().await?;

    let typeface_files: HashMap<TypefaceType, Vec<u8>> =
        get_typeface_files(&typeface_file_urls).await?;
    typeface_files
        .iter()
        .for_each(|(typeface_type, bytes)| crate::typeface::load_typeface(&typeface_type, bytes));

    Ok(())
}

async fn load_typeface_file_urls_file() -> Result<TypefaceFileUrlsFile, Box<dyn std::error::Error>>
{
    let url = "resources/font/map.json";
    Ok(fetch_get_json(url).await?)
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
    let iter = join_all(typeface_file_urls.into_iter().map(
        |(typeface_type, font_file_url)| async move {
            let bytes = fetch_get_vec_u8(font_file_url)
                .await
                .expect(format!("Could not fetch {}", font_file_url).as_str());
            (*typeface_type, bytes)
        },
    ))
    .await;

    Ok(HashMap::from_iter(iter))
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
