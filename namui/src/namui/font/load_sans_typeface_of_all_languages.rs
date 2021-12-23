use crate::{
    fetch_get_json, fetch_get_vec_u8,
    namui::{self, manager::TypefaceManager, FontWeight, Language, TypefaceType},
};
use futures::future::join_all;
use std::{collections::HashMap, iter::FromIterator};

type TypefaceFileUrls = HashMap<TypefaceType, String>;
type TypefaceFileUrlsFile = HashMap<Language, HashMap<FontWeight, String>>;

pub async fn load_sans_typeface_of_all_languages(
    typeface_manager: &mut TypefaceManager,
) -> Result<(), String> {
    let typeface_file_urls: TypefaceFileUrls = get_typeface_file_urls().await?;

    let typeface_files: HashMap<TypefaceType, Vec<u8>> =
        get_typeface_files(&typeface_file_urls).await;
    typeface_files
        .iter()
        .for_each(|(typeface_type, bytes)| typeface_manager.load_typeface(&typeface_type, bytes));

    Ok(())
}

async fn load_typeface_file_urls_file() -> Result<TypefaceFileUrlsFile, String> {
    let url = "resources/font/map.json";
    fetch_get_json(url).await
}

async fn get_typeface_file_urls() -> Result<TypefaceFileUrls, String> {
    let typeface_file_map_file: TypefaceFileUrlsFile = load_typeface_file_urls_file().await?;

    Ok(typeface_file_map_file
        .iter()
        .flat_map(|(language, font_file_map)| {
            font_file_map.iter().map(move |(font_weight, font_file_url)| {
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
) -> HashMap<TypefaceType, Vec<u8>> {
    let iter =
        join_all(typeface_file_urls.into_iter().map(|(typeface_type, font_file_url)| async move {
            let bytes = fetch_get_vec_u8(font_file_url).await.unwrap();
            (*typeface_type, bytes)
        }))
        .await;

    return HashMap::from_iter(iter);
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use std::collections::HashMap;

    use crate::namui::{
        font::load_sans_typeface_of_all_languages::TypefaceFileUrlsFile, FontWeight, Language,
    };

    #[test]
    #[wasm_bindgen_test]
    fn font_file_url_map_should_be_serializable() {
        let font_file_url_map: TypefaceFileUrlsFile = serde_json::from_str(
            r#"
        {
            "Ko": {
                "100": "ko/NotoSansKR-Thin.otf",
                "300": "ko/NotoSansKR-Light.otf",
                "400": "ko/NotoSansKR-Regular.otf",
                "500": "ko/NotoSansKR-Medium.otf",
                "700": "ko/NotoSansKR-Bold.otf",
                "900": "ko/NotoSansKR-Black.otf"
            }
        }"#,
        )
        .unwrap();

        assert_eq!(
            font_file_url_map,
            HashMap::from([(
                Language::Ko,
                HashMap::from([
                    (FontWeight::_100, "ko/NotoSansKR-Thin.otf".to_string()),
                    (FontWeight::_300, "ko/NotoSansKR-Light.otf".to_string()),
                    (FontWeight::_400, "ko/NotoSansKR-Regular.otf".to_string()),
                    (FontWeight::_500, "ko/NotoSansKR-Medium.otf".to_string()),
                    (FontWeight::_700, "ko/NotoSansKR-Bold.otf".to_string()),
                    (FontWeight::_900, "ko/NotoSansKR-Black.otf".to_string()),
                ])
            )])
        );

        let serialized_font_file_url_map = serde_json::to_string(&font_file_url_map).unwrap();
        assert_eq!(serialized_font_file_url_map, "{\"Ko\":{\"100\":\"ko/NotoSansKR-Thin.otf\",\"300\":\"ko/NotoSansKR-Light.otf\",\"400\":\"ko/NotoSansKR-Regular.otf\",\"500\":\"ko/NotoSansKR-Medium.otf\",\"700\":\"ko/NotoSansKR-Bold.otf\",\"900\":\"ko/NotoSansKR-Black.otf\"}}");
    }
}
