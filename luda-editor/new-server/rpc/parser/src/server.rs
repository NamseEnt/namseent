use crate::*;
use proc_macro2::TokenStream;
use quote::quote;

impl Rpc {
    pub fn server_handler(&self) -> TokenStream {
        let mut api_index = 0;
        let rpc_structs = self.services.iter().map(|service| {
            let service_name = &service.name;
            let chunks = service.apis.iter().map(|api| {
                let this_api_index = api_index;
                api_index += 1;

                let api_name = &api.name;

                quote! {
                    #this_api_index => {
                        let Ok(request) = rkyv::validation::validators::check_archived_root::<
                            api::#service_name::#api_name::Request,
                        >(in_payload) else {
                            return Err(anyhow::anyhow!("Failed to validate packet"));
                        };
                        match api::#service_name::#api_name::#api_name(request, db, session)
                            .await
                            .and_then(|response| Ok(rkyv::to_bytes::<_, 64>(&response)?))
                        {
                            Ok(bytes) => (bytes.into_vec(), Status::Ok),
                            Err(error) => {
                                eprintln!("Error on #api_name: {:?}", error);
                                (Vec::new(), Status::ServerError)
                            }
                        }
                    }
                }
            });
            quote! {
                #(#chunks)*
            }
        });

        quote! {
            #(#rpc_structs)*
        }
    }
}

// 0 => {
//     let Ok(request) = rkyv::validation::validators::check_archived_root::<
//         api::google_auth::Request,
//     >(in_payload) else {
//         return Err(anyhow::anyhow!("Failed to validate packet"));
//     };
//     match api::google_auth::google_auth(request, db, session)
//         .await
//         .and_then(|response| Ok(rkyv::to_bytes::<_, 64>(&response)?))
//     {
//         Ok(bytes) => (bytes.into_vec(), Status::Ok),
//         Err(error) => {
//             eprintln!("Error on google_auth: {:?}", error);
//             (Vec::new(), Status::ServerError)
//         }
//     }
// }
