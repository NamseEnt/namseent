use super::*;
use syn::{parse::Parse, punctuated::Punctuated, Ident, Token};

pub struct Service {
    pub name: Ident,
    pub methods: Vec<Method>,
}

impl Parse for Service {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![:]>()?;

        let methods_content;
        syn::braced!(methods_content in input);
        let methods = Punctuated::<Method, Token![,]>::parse_terminated(&methods_content)
            .map(|methods| methods.into_iter().collect())?;

        Ok(Self { name, methods })
    }
}

impl Service {
    pub fn service_trait(&self) -> proc_macro2::TokenStream {
        let service_name_in_snake_case = to_snake_case(&self.name);
        let methods = self.methods.iter().map(|method| match method {
            Method::QueueMethod(method) => {
                let method_name = &method.name;
                quote! {
                    fn #method_name<'a>(
                        &'a self,
                        session: Option<TSession>,
                        req: super::#service_name_in_snake_case::#method_name::Request,
                    ) -> std::pin::Pin<
                        Box<dyn 'a + std::future::Future<Output = super::#service_name_in_snake_case::#method_name::Result> + Send>,
                    >;
                }
            },
            Method::RequestAndResponseMethod(method) => {
                let method_name = &method.name;
                quote! {
                    fn #method_name<'a>(
                        &'a self,
                        session: Option<TSession>,
                        req: super::#service_name_in_snake_case::#method_name::Request,
                    ) -> std::pin::Pin<
                        Box<dyn 'a + std::future::Future<Output = super::#service_name_in_snake_case::#method_name::Result> + Send>,
                    >;
                }
            }
        });
        let service_name = &self.name;
        quote! {
            pub trait #service_name<TSession> {
                #(#methods)*
            }
        }
    }
}
