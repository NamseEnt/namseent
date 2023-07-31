use super::*;

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
        let id = startExecuteAsyncFunction(
            self.args_names.into_iter().map(JsValue::from).collect(),
            self.code,
            self.args.into_boxed_slice(),
        );

        let notify = Arc::new(Notify::new());

        unsafe {
            ASYNC_FUNCTION_RESULT_NOTIFY_MAP
                .get_mut()
                .unwrap()
                .insert(id, notify.clone());
        };

        notify.notified().await;

        let result = unsafe {
            ASYNC_FUNCTION_RESULT_MAP
                .get_mut()
                .unwrap()
                .remove(&id)
                .unwrap()
        };

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

pub(crate) fn on_async_function_executed(id: usize) {
    let result = getAsyncFunctionResult(id);

    unsafe {
        ASYNC_FUNCTION_RESULT_MAP
            .get_mut()
            .unwrap()
            .insert(id, result);

        ASYNC_FUNCTION_RESULT_NOTIFY_MAP
            .get_mut()
            .unwrap()
            .remove(&id)
            .unwrap()
            .notify_one();
    };
}
