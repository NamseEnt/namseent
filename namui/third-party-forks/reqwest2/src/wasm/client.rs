use http::header::USER_AGENT;
use http::{HeaderMap, HeaderValue, Method};
use std::convert::TryInto;
use std::{fmt, future::Future, sync::Arc};

use super::{Request, RequestBuilder, Response};
use crate::IntoUrl;

/// dox
#[derive(Clone)]
pub struct Client {
    config: Arc<Config>,
}

/// dox
pub struct ClientBuilder {
    config: Config,
}

impl Client {
    /// dox
    pub fn new() -> Self {
        Client::builder().build().unwrap()
    }

    /// dox
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Convenience method to make a `GET` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever supplied `Url` cannot be parsed.
    pub fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::GET, url)
    }

    /// Convenience method to make a `POST` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever supplied `Url` cannot be parsed.
    pub fn post<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::POST, url)
    }

    /// Convenience method to make a `PUT` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever supplied `Url` cannot be parsed.
    pub fn put<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::PUT, url)
    }

    /// Convenience method to make a `PATCH` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever supplied `Url` cannot be parsed.
    pub fn patch<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::PATCH, url)
    }

    /// Convenience method to make a `DELETE` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever supplied `Url` cannot be parsed.
    pub fn delete<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::DELETE, url)
    }

    /// Convenience method to make a `HEAD` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever supplied `Url` cannot be parsed.
    pub fn head<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::HEAD, url)
    }

    /// Start building a `Request` with the `Method` and `Url`.
    ///
    /// Returns a `RequestBuilder`, which will allow setting headers and
    /// request body before sending.
    ///
    /// # Errors
    ///
    /// This method fails whenever supplied `Url` cannot be parsed.
    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        let req = url.into_url().map(move |url| Request::new(method, url));
        RequestBuilder::new(self.clone(), req)
    }

    /// Executes a `Request`.
    ///
    /// A `Request` can be built manually with `Request::new()` or obtained
    /// from a RequestBuilder with `RequestBuilder::build()`.
    ///
    /// You should prefer to use the `RequestBuilder` and
    /// `RequestBuilder::send()`.
    ///
    /// # Errors
    ///
    /// This method fails if there was an error while sending request,
    /// redirect loop was detected or redirect limit was exhausted.
    pub fn execute(
        &self,
        request: Request,
    ) -> impl Future<Output = Result<Response, crate::Error>> {
        self.execute_request(request)
    }

    // merge request headers with Client default_headers, prior to external http fetch
    fn merge_headers(&self, req: &mut Request) {
        use http::header::Entry;
        let headers: &mut HeaderMap = req.headers_mut();
        // insert default headers in the request headers
        // without overwriting already appended headers.
        for (key, value) in self.config.headers.iter() {
            if let Entry::Vacant(entry) = headers.entry(key) {
                entry.insert(value.clone());
            }
        }
    }

    pub(super) fn execute_request(
        &self,
        mut req: Request,
    ) -> impl Future<Output = crate::Result<Response>> {
        self.merge_headers(&mut req);
        fetch(req)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("Client");
        self.config.fmt_fields(&mut builder);
        builder.finish()
    }
}

impl fmt::Debug for ClientBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("ClientBuilder");
        self.config.fmt_fields(&mut builder);
        builder.finish()
    }
}

async fn fetch(req: Request) -> crate::Result<Response> {
    todo!()
    // // Build the js Request
    // let mut init = web_sys::RequestInit::new();
    // init.method(req.method().as_str());

    // // convert HeaderMap to Headers
    // let js_headers = web_sys::Headers::new()
    //     .map_err(crate::error::wasm)
    //     .map_err(crate::error::builder)?;

    // for (name, value) in req.headers() {
    //     js_headers
    //         .append(
    //             name.as_str(),
    //             value.to_str().map_err(crate::error::builder)?,
    //         )
    //         .map_err(crate::error::wasm)
    //         .map_err(crate::error::builder)?;
    // }
    // init.headers(&js_headers.into());

    // // When req.cors is true, do nothing because the default mode is 'cors'
    // if !req.cors {
    //     init.mode(web_sys::RequestMode::NoCors);
    // }

    // if let Some(creds) = req.credentials {
    //     init.credentials(creds);
    // }

    // if let Some(body) = req.body() {
    //     if !body.is_empty() {
    //         init.body(Some(body.to_js_value()?.as_ref()));
    //     }
    // }

    // let abort = AbortGuard::new()?;
    // init.signal(Some(&abort.signal()));

    // let js_req = web_sys::Request::new_with_str_and_init(req.url().as_str(), &init)
    //     .map_err(crate::error::wasm)
    //     .map_err(crate::error::builder)?;

    // // Await the fetch() promise
    // let p = js_fetch(&js_req);
    // let js_resp = super::promise::<web_sys::Response>(p)
    //     .await
    //     .map_err(crate::error::request)?;

    // // Convert from the js Response
    // let mut resp = http::Response::builder().status(js_resp.status());

    // let url = Url::parse(&js_resp.url()).expect_throw("url parse");

    // let js_headers = js_resp.headers();
    // let js_iter = js_sys::try_iter(&js_headers)
    //     .expect_throw("headers try_iter")
    //     .expect_throw("headers have an iterator");

    // for item in js_iter {
    //     let item = item.expect_throw("headers iterator doesn't throw");
    //     let serialized_headers: String = JSON::stringify(&item)
    //         .expect_throw("serialized headers")
    //         .into();
    //     let [name, value]: [String; 2] = serde_json::from_str(&serialized_headers)
    //         .expect_throw("deserializable serialized headers");
    //     resp = resp.header(&name, &value);
    // }

    // resp.body(js_resp)
    //     .map(|resp| Response::new(resp, url, abort))
    //     .map_err(crate::error::request)
}

// ===== impl ClientBuilder =====

impl ClientBuilder {
    /// dox
    pub fn new() -> Self {
        ClientBuilder {
            config: Config::default(),
        }
    }

    /// Returns a 'Client' that uses this ClientBuilder configuration
    pub fn build(mut self) -> Result<Client, crate::Error> {
        if let Some(err) = self.config.error {
            return Err(err);
        }

        let config = std::mem::take(&mut self.config);
        Ok(Client {
            config: Arc::new(config),
        })
    }

    /// Sets the `User-Agent` header to be used by this client.
    pub fn user_agent<V>(mut self, value: V) -> ClientBuilder
    where
        V: TryInto<HeaderValue>,
        V::Error: Into<http::Error>,
    {
        match value.try_into() {
            Ok(value) => {
                self.config.headers.insert(USER_AGENT, value);
            }
            Err(e) => {
                self.config.error = Some(crate::error::builder(e.into()));
            }
        }
        self
    }

    /// Sets the default headers for every request
    pub fn default_headers(mut self, headers: HeaderMap) -> ClientBuilder {
        for (key, value) in headers.iter() {
            self.config.headers.insert(key, value.clone());
        }
        self
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Config {
    headers: HeaderMap,
    error: Option<crate::Error>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            headers: HeaderMap::new(),
            error: None,
        }
    }
}

impl Config {
    fn fmt_fields(&self, f: &mut fmt::DebugStruct<'_, '_>) {
        f.field("default_headers", &self.headers);
    }
}
