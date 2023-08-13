use super::*;
use crate::*;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;

pub(super) async fn init() -> InitResult {
    Ok(())
}

pub struct ExecuteAsyncFunctionBuilder {
    args_names: Vec<String>,
    code: String,
    args: Vec<JsValue>,
}

impl ExecuteAsyncFunctionBuilder {
    fn new(code: impl AsRef<str>) -> Self {
        Self {
            args_names: Vec::new(),
            code: code.as_ref().to_string(),
            args: Vec::new(),
        }
    }
    pub fn arg(mut self, name: impl AsRef<str>, arg: impl serde::Serialize) -> Self {
        self.args_names.push(name.as_ref().to_string());
        self.args.push(serde_wasm_bindgen::to_value(&arg).unwrap());
        self
    }
    pub async fn run<T: DeserializeOwned>(self) -> T {
        let js_func = js_sys::Function::new_with_args(&self.args_names.join(","), &self.code);

        let js_args = {
            let array = js_sys::Array::new_with_length(self.args.len() as u32);
            for arg in self.args {
                array.push(&arg);
            }
            array
        };
        let promise: js_sys::Promise = js_func
            .apply(&wasm_bindgen::JsValue::NULL, &js_args)
            .unwrap()
            .into();

        let result = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
        serde_wasm_bindgen::from_value(result).unwrap()
    }
}

pub fn execute_async_function(code: impl AsRef<str>) -> ExecuteAsyncFunctionBuilder {
    ExecuteAsyncFunctionBuilder::new(format!(
        "
    return (async () => {{
        {}
    }})();
    ",
        code.as_ref()
    ))
}
