use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

pub async fn fetch_get(url: &str) -> Result<Response, String> {
    let mut options = RequestInit::new();
    options.method("GET");

    let request = Request::new_with_str_and_init(url, &options).unwrap();

    let window = web_sys::window().unwrap();
    match JsFuture::from(window.fetch_with_request(&request)).await {
        Ok(response_value) => {
            assert!(response_value.is_instance_of::<Response>());
            let response: Response = response_value.dyn_into().unwrap();

            if !response.ok() {
                return Err(response.status_text());
            }
            Result::Ok(response)
        }
        Err(error) => return Err(format!("fail to fetch {} - {:?}", url, error)),
    }
}

pub async fn fetch_get_array_buffer(url: &str) -> Result<ArrayBuffer, String> {
    let response: Response = fetch_get(&url).await?;

    let array_buffer = JsFuture::from(response.array_buffer().unwrap())
        .await
        .unwrap()
        .dyn_into()
        .unwrap();

    Result::Ok(array_buffer)
}

pub async fn fetch_get_json<T: for<'a> serde::Deserialize<'a>>(url: &str) -> Result<T, String> {
    let response: Response = fetch_get(&url).await?;

    let json = JsFuture::from(response.json().unwrap()).await.unwrap();

    json.into_serde().map_err(|e| e.to_string())
}

pub async fn fetch_get_vec_u8(url: &str) -> Result<Vec<u8>, String> {
    let array_buffer = fetch_get_array_buffer(url).await?;
    let array_buffer_view = Uint8Array::new(&array_buffer);
    let bytes = array_buffer_view.to_vec();
    Result::Ok(bytes)
}
