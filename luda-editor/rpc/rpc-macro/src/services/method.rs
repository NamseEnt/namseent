use super::*;
use syn::{parse::Parse, Ident};

pub enum Method {
    QueueMethod(QueueMethod),
    RequestAndResponseMethod(RequestAndResponseMethod),
}
impl Parse for Method {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if !input.peek(Ident) {
            return Err(input.error("expected ident"));
        }

        if input.peek2(Ident) {
            let method = input.parse::<QueueMethod>()?;
            Ok(Self::QueueMethod(method))
        } else {
            let method = input.parse::<RequestAndResponseMethod>()?;
            Ok(Self::RequestAndResponseMethod(method))
        }
    }
}
