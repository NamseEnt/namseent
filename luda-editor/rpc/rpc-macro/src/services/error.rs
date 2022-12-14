use syn::{parse::Parse, punctuated::Punctuated, Ident, Token, Variant};

pub struct ErrorDef {
    pub variants: Punctuated<Variant, Token![,]>,
}
impl Parse for ErrorDef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        if name != "Error" {
            return Err(syn::Error::new_spanned(name, "expected `Error`"));
        }

        let content;
        syn::braced!(content in input);
        let variants = content.parse_terminated(Variant::parse)?;

        Ok(Self { variants })
    }
}
