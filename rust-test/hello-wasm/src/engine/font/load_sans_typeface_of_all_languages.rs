use crate::engine::{
    manager::TypefaceManager, Engine, EngineImpl, FontWeight, Language, TypefaceType,
};
use futures::{future::join_all, stream, StreamExt};
use js_sys::{ArrayBuffer, Uint8Array};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter::FromIterator};
use strum::IntoEnumIterator;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

type TypefaceFileUrls = HashMap<TypefaceType, String>;
type TypefaceFileUrlsFile = HashMap<Language, HashMap<FontWeight, String>>;

pub async fn load_sans_typeface_of_all_languages(typeface_manager: &mut dyn TypefaceManager) {
    let typeface_file_urls: TypefaceFileUrls = get_typeface_file_urls().await;

    let typeface_files: HashMap<TypefaceType, Vec<u8>> =
        get_typeface_files(&typeface_file_urls).await;

    typeface_files
        .iter()
        .for_each(|(typeface_type, bytes)| typeface_manager.load_typeface(&typeface_type, bytes));
}

async fn fetch_get(url: &str) -> Result<Response, String> {
    let mut options = RequestInit::new();
    options.method("GET");

    let request = Request::new_with_str_and_init(url, &options).unwrap();

    let window = web_sys::window().unwrap();
    let response_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();
    assert!(response_value.is_instance_of::<Response>());
    let response: Response = response_value.dyn_into().unwrap();

    if !response.ok() {
        Engine::log(format!("Failed to fetch {}", url));
        return Err(response.status_text());
    }
    Result::Ok(response)
}

async fn fetch_get_array_buffer(url: &str) -> Result<ArrayBuffer, String> {
    let response: Response = fetch_get(&url).await.unwrap();

    let array_buffer = JsFuture::from(response.array_buffer().unwrap())
        .await
        .unwrap()
        .dyn_into()
        .unwrap();

    Result::Ok(array_buffer)
}

async fn fetch_get_json<T: for<'a> serde::Deserialize<'a>>(url: &str) -> Result<T, String> {
    let response: Response = fetch_get(&url).await.unwrap();

    let json = JsFuture::from(response.json().unwrap()).await.unwrap();

    json.into_serde().map_err(|e| e.to_string())
}

async fn load_typeface_file_urls_file() -> TypefaceFileUrlsFile {
    let url = "engine/resources/font/map.json";
    fetch_get_json(url).await.unwrap()
}

async fn get_typeface_file_urls() -> TypefaceFileUrls {
    let typeface_file_map_file: TypefaceFileUrlsFile = load_typeface_file_urls_file().await;

    typeface_file_map_file
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
        .collect()
}

async fn get_typeface_files(
    typeface_file_urls: &TypefaceFileUrls,
) -> HashMap<TypefaceType, Vec<u8>> {
    let iter = join_all(typeface_file_urls.into_iter().map(
        |(typeface_type, font_file_url)| async move {
            let array_buffer = fetch_get_array_buffer(font_file_url).await.unwrap();
            let array_buffer_view = Uint8Array::new(&array_buffer);
            let bytes = array_buffer_view.to_vec();
            (*typeface_type, bytes)
        },
    ))
    .await;

    return HashMap::from_iter(iter);
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use std::collections::HashMap;

    use crate::engine::{
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
                "350": "ko/NotoSansKR-DemiLight.otf",
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
                    (FontWeight(100), "ko/NotoSansKR-Thin.otf".to_string()),
                    (FontWeight(300), "ko/NotoSansKR-Light.otf".to_string()),
                    (FontWeight(350), "ko/NotoSansKR-DemiLight.otf".to_string()),
                    (FontWeight(400), "ko/NotoSansKR-Regular.otf".to_string()),
                    (FontWeight(500), "ko/NotoSansKR-Medium.otf".to_string()),
                    (FontWeight(700), "ko/NotoSansKR-Bold.otf".to_string()),
                    (FontWeight(900), "ko/NotoSansKR-Black.otf".to_string()),
                ])
            )])
        );

        let serialized_font_file_url_map = serde_json::to_string(&font_file_url_map).unwrap();
        assert_eq!(serialized_font_file_url_map, "{\"Ko\":{\"100\":\"ko/NotoSansKR-Thin.otf\",\"300\":\"ko/NotoSansKR-Light.otf\",\"400\":\"ko/NotoSansKR-Regular.otf\",\"500\":\"ko/NotoSansKR-Medium.otf\",\"700\":\"ko/NotoSansKR-Bold.otf\",\"900\":\"ko/NotoSansKR-Black.otf\",\"350\":\"ko/NotoSansKR-DemiLight.otf\"}}");
    }
}
