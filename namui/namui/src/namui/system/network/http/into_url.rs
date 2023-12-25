use url::*;

pub trait IntoUrl {
    fn into_url(self) -> Result<Url, ParseError>;
    fn as_str(&self) -> &str;
}

impl IntoUrl for Url {
    fn into_url(self) -> Result<Url, ParseError> {
        Ok(self)
    }
    fn as_str(&self) -> &str {
        self.as_str()
    }
}
impl IntoUrl for String {
    fn into_url(self) -> Result<Url, ParseError> {
        Url::parse(&self)
    }
    fn as_str(&self) -> &str {
        self
    }
}
impl<'a> IntoUrl for &'a str {
    fn into_url(self) -> Result<Url, ParseError> {
        Url::parse(self)
    }
    fn as_str(&self) -> &str {
        self
    }
}
impl<'a> IntoUrl for &'a String {
    fn into_url(self) -> Result<Url, ParseError> {
        Url::parse(self)
    }
    fn as_str(&self) -> &str {
        self
    }
}
