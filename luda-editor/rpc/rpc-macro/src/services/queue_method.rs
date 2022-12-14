use super::*;
use syn::{parse::Parse, Ident, Token};

pub struct QueueMethod {
    pub name: Ident,
    pub request: syn::ItemStruct,
    pub error: ErrorDef,
}
impl Parse for QueueMethod {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let queue_sign = input.parse::<Ident>()?;
        if queue_sign != "QUEUE" {
            return Err(input.error("expected `QUEUE`"));
        }

        let name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let content;
        syn::braced!(content in input);
        let request = content.parse::<syn::ItemStruct>()?;
        match request.vis {
            syn::Visibility::Public(_) => {}
            _ => return Err(syn::Error::new_spanned(request, "expected `pub struct`")),
        };
        let error = content.parse::<ErrorDef>()?;
        Ok(Self {
            name,
            request,
            error,
        })
    }
}
