use super::*;

pub struct ExecuteFunctionSyncBuilder {
    args_names: Vec<String>,
    code: String,
    args: Vec<JsValue>,
}

impl ExecuteFunctionSyncBuilder {
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
        serde_wasm_bindgen::from_value(executeFunctionSyncOnMain(
            self.args_names.into_iter().map(JsValue::from).collect(),
            self.code,
            self.args,
        ))
        .unwrap()
    }
}

pub fn execute_function_sync(code: impl AsRef<str>) -> ExecuteFunctionSyncBuilder {
    ExecuteFunctionSyncBuilder::new(code)
}
