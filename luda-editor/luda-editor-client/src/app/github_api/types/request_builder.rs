use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

pub struct RequestBuilder {
    url: String,
    method: RequestMethod,
    request_content_type: RequestContentType,
    response_content_type: ResponseContentType,
    access_token: Option<String>,
    body: Option<JsValue>,
}
impl RequestBuilder {
    pub fn new(url: String) -> Self {
        Self {
            url,
            method: RequestMethod::Get,
            request_content_type: RequestContentType::None,
            response_content_type: ResponseContentType::None,
            access_token: None,
            body: None,
        }
    }

    pub fn get(&mut self) -> &mut Self {
        self.method = RequestMethod::Get;
        self
    }
    pub fn put(&mut self) -> &mut Self {
        self.method = RequestMethod::Put;
        self
    }
    pub fn post(&mut self) -> &mut Self {
        self.method = RequestMethod::Post;
        self
    }
    pub fn delete(&mut self) -> &mut Self {
        self.method = RequestMethod::Delete;
        self
    }

    pub fn access_token(&mut self, access_token: String) -> &mut Self {
        self.access_token = Some(access_token);
        self
    }

    pub fn json_body<T>(&mut self, body: &T) -> &mut Self
    where
        T: serde::Serialize,
    {
        self.request_content_type = RequestContentType::Json;
        self.body = Some(JsValue::from_str(
            serde_json::to_string(&body).unwrap().as_str(),
        ));
        self
    }

    pub fn accept_none(&mut self) -> &mut Self {
        self.response_content_type = ResponseContentType::None;
        self
    }
    pub fn accept_json(&mut self) -> &mut Self {
        self.response_content_type = ResponseContentType::Json;
        self
    }
    pub fn accept_raw(&mut self) -> &mut Self {
        self.response_content_type = ResponseContentType::Raw;
        self
    }

    pub async fn send(&self) -> Response {
        let mut opts = RequestInit::new();
        opts.method(match self.method {
            RequestMethod::Get => "GET",
            RequestMethod::Put => "PUT",
            RequestMethod::Post => "POST",
            RequestMethod::Delete => "DELETE",
        });
        if self.body.is_some() && self.method != RequestMethod::Get {
            opts.body(self.body.as_ref());
        }

        let request = Request::new_with_str_and_init(&self.url, &opts).unwrap();
        match self.request_content_type {
            RequestContentType::Json => request
                .headers()
                .set("Content-Type", "application/json")
                .unwrap(),
            _ => {}
        };
        match self.response_content_type {
            ResponseContentType::Json => request
                .headers()
                .set("Accept", "application/vnd.github.v3+json")
                .unwrap(),
            ResponseContentType::Raw => request
                .headers()
                .set("Accept", "application/vnd.github.v3+raw")
                .unwrap(),
            _ => {}
        }
        if let Some(access_token) = &self.access_token {
            request
                .headers()
                .set("Authorization", &format!("token {}", access_token))
                .unwrap();
        }

        let window = web_sys::window().unwrap();
        let response = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();
        response.dyn_into().unwrap()
    }
}

#[derive(PartialEq)]
enum RequestMethod {
    Get,
    Put,
    Post,
    Delete,
}

enum RequestContentType {
    None,
    Json,
}

enum ResponseContentType {
    None,
    Json,
    Raw,
}
