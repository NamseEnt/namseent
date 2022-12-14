mod client_rpc_impl;
mod rpc_structs;
mod server_handle_rpc;

use super::*;
use client_rpc_impl::*;
use rpc_structs::*;
use server_handle_rpc::*;
crate::RPC.auth_service.exchange_google_auth_code_to_access_token(); 으로 할 수 있게 해줘. 부탁해~
impl Services {
    pub fn to_tokens(&self) -> proc_macro2::TokenStream {
        let rpc_structs = rpc_structs(&self);
        let service_traits = self.services.iter().map(|service| service.service_trait());
        let server_handle_rpc = server_handle_rpc(&self);
        let client_rpc_impl = client_rpc_impl(&self);

        quote! {
            #rpc_structs

            #[cfg(feature = "server")]
            mod server {
                pub use hyper;
                #(#service_traits)*

                #server_handle_rpc
            }
            #[cfg(feature = "server")]
            pub use server::*;

            #[cfg(feature = "client")]
            mod client {
                use std::sync::Mutex;

                pub struct RpcSetting {
                    endpoint: String,
                    session_id: Option<uuid::Uuid>,
                }

                pub struct Rpc {
                    setting: Mutex<RpcSetting>,
                }

                impl Rpc {
                    pub const fn new(endpoint: String) -> Self {
                        Self {
                            setting: Mutex::new(RpcSetting {
                                endpoint,
                                session_id: None,
                            }),
                        }
                    }
                    pub fn set_session_id(&self, session_id: uuid::Uuid) {
                        let mut setting = self.setting.lock().unwrap();
                        setting.session_id.replace(session_id);
                    }
                    pub fn session_id(&self) -> Option<uuid::Uuid> {
                        let setting = self.setting.lock().unwrap();
                        setting.session_id
                    }
                    pub fn set_endpoint(&self, endpoint: String) {
                        let mut setting = self.setting.lock().unwrap();
                        setting.endpoint = endpoint;
                    }
                    pub fn endpoint(&self) -> String {
                        let setting = self.setting.lock().unwrap();
                        setting.endpoint.clone()
                    }
                }

                #client_rpc_impl
            }
            #[cfg(feature = "client")]
            pub use client::*;
        }
    }
}
