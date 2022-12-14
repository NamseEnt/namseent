use super::*;

pub fn rpc_structs(services: &Services) -> proc_macro2::TokenStream {
    let service_mods = services.services.iter().map(|service| {
        let service_name_in_snake_case = to_snake_case(&service.name);
        let methods = service.methods.iter().map(|method| match method {
            Method::QueueMethod(_) => todo!(),
            Method::RequestAndResponseMethod(method) => {
                let method_name = &method.name;
                let request = struct_output(&method.request);
                let response = struct_output(&method.response);
                let other_structs = method
                    .other_structs
                    .iter()
                    .map(|other_struct| struct_output(other_struct));
                let other_enums = method
                    .other_enums
                    .iter()
                    .map(|other_enum| enum_output(other_enum));
                let error = error_output(&method.error);

                quote! {
                    pub mod #method_name {
                        #request
                        #response
                        #(#other_structs)*
                        #(#other_enums)*
                        #error

                        simple_error_impl!(Error);

                        pub type Result = core::result::Result<Response, Error>;
                    }
                }
            }
        });

        quote! {
            pub mod #service_name_in_snake_case {
                #(#methods)*
            }
        }
    });

    quote! {
        #(#service_mods)*
    }
}

fn struct_output(item_struct: &syn::ItemStruct) -> proc_macro2::TokenStream {
    quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #item_struct
    }
}

fn enum_output(item_enum: &syn::ItemEnum) -> proc_macro2::TokenStream {
    quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #item_enum
    }
}

fn error_output(error_def: &ErrorDef) -> proc_macro2::TokenStream {
    let variants = error_def.variants.iter();
    quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub enum Error {
            #(#variants),*
        }
    }
}
