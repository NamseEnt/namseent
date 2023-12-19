use super::*;

pub struct ExecuteFunctionBuilder {
    args_names: Vec<String>,
    code: String,
    args: Vec<JsValue>,
}

impl ExecuteFunctionBuilder {
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
    pub fn run<T: DeserializeOwned>(self) -> T {
        let js_func = js_sys::Function::new_with_args(&self.args_names.join(","), &self.code);

        let js_args = {
            let array = js_sys::Array::new();
            for arg in self.args {
                array.push(&arg);
            }
            array
        };

        let result = js_func
            .apply(&wasm_bindgen::JsValue::NULL, &js_args)
            .unwrap();
        serde_wasm_bindgen::from_value(result).unwrap()
    }
}

pub fn execute_function(code: impl AsRef<str>) -> ExecuteFunctionBuilder {
    ExecuteFunctionBuilder::new(code)
}
