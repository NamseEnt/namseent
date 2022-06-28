use core::fmt;
use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

pub async fn fetch_get(url: &str) -> Result<Response, FetchError> {
    let mut options = RequestInit::new();
    options.method("GET");

    let request = Request::new_with_str_and_init(url, &options)?;

    let window = web_sys::window().unwrap();
    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let response: Response = response_value.dyn_into()?;

    if !response.ok() {
        return Err(FetchError {
            message: response.status_text(),
        });
    }
    Result::Ok(response)
}

pub async fn fetch_get_array_buffer(url: &str) -> Result<ArrayBuffer, FetchError> {
    let response: Response = fetch_get(&url).await?;

    let array_buffer = JsFuture::from(response.array_buffer()?).await?.dyn_into()?;

    Result::Ok(array_buffer)
}

pub async fn fetch_get_json<T: for<'a> serde::Deserialize<'a>>(url: &str) -> Result<T, FetchError> {
    let response: Response = fetch_get(&url).await?;

    let json = JsFuture::from(response.json()?).await?;

    json.into_serde().map_err(|e| FetchError {
        message: format!("fail to deserialize for {} - {}", url, e),
    })
}

pub async fn fetch_get_vec_u8(url: &str) -> Result<Vec<u8>, FetchError> {
    let array_buffer = fetch_get_array_buffer(url).await?;
    let array_buffer_view = Uint8Array::new(&array_buffer);
    let bytes = array_buffer_view.to_vec();
    Result::Ok(bytes)
}

#[derive(Debug)]
pub struct FetchError {
    message: String,
}

impl From<JsValue> for FetchError {
    fn from(js_value: JsValue) -> Self {
        Self {
            message: format!("{:?}", js_value),
        }
    }
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FetchError {}
