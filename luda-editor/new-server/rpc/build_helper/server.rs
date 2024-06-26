use crate::*;
use build_helper::write_fmt;
use quote::quote;

pub fn generate_code(rpc: &Rpc) {
    generate_api_files(rpc);
    generate_api_wire_up_file(rpc);
}

fn server_src_path() -> &'static std::path::Path {
    std::path::Path::new("../server/src")
}

fn generate_api_files(rpc: &Rpc) {
    let api_path = server_src_path().join("api");
    let mut api_mod_rs_lines = rpc
        .services
        .iter()
        .map(|service| format!("pub mod {};\n", service.snake_case_name,))
        .collect::<Vec<String>>();
    api_mod_rs_lines.sort();
    std::fs::write(api_path.join("mod.rs"), api_mod_rs_lines.join("")).unwrap();

    for service in &rpc.services {
        let service_snake_name = &service.snake_case_name;
        let service_path = api_path.join(service_snake_name.to_string());
        if !service_path.exists() {
            std::fs::create_dir_all(&service_path).unwrap();
        }

        let mut service_mod_rs_lines = service
            .apis
            .iter()
            .map(|api| format!("pub mod {};\n", api.name))
            .collect::<Vec<String>>();
        service_mod_rs_lines.sort();
        std::fs::write(service_path.join("mod.rs"), service_mod_rs_lines.join("")).unwrap();

        for api in &service.apis {
            let api_dir = service_path.join(format!("{}", api.name));
            if !api_dir.exists() {
                std::fs::create_dir_all(&api_dir).unwrap();
            }
            let api_mod_rs = api_dir.join("mod.rs");
            if api_mod_rs.exists() {
                continue;
            }
            let api_name = &api.name;
            std::fs::write(
                &api_mod_rs,
                format!(
                    "

use crate::*;
use database::schema::*;
use luda_rpc::{service_snake_name}::{api_name}::*;

pub async fn {api_name}(
    ArchivedRequest {{ }}: &ArchivedRequest,
    db: Database,
    session: Session,
) -> Result<Response, Error> {{
    todo!()
}}
"
                ),
            )
            .unwrap();
        }
    }
}

fn generate_api_wire_up_file(rpc: &Rpc) {
    let handle_file_path = server_src_path().join("ws_handler/handle.rs");

    let mut api_index: u16 = 0;
    let api_matches = rpc.services.iter().map(|service| {
        let service_name = &service.snake_case_name;
        let apis = service.apis.iter().map(|api| {
            let api_name = &api.name;
            let this_api_index = api_index;
            api_index += 1;
            quote! {
                #this_api_index => {
                    let Ok(request) = rkyv::validation::validators::check_archived_root::<
                        luda_rpc::#service_name::#api_name::Request,
                    >(in_payload) else {
                        return Err(anyhow::anyhow!("Failed to validate packet"));
                    };
                    match api::#service_name::#api_name::#api_name(request, db, session)
                        .await
                    {
                        Ok(response) => Ok(HandleResult::Response(
                            rkyv::to_bytes::<_, 64>(&response)?.to_vec()
                        )),
                        Err(error) => Ok(HandleResult::Error(
                            rkyv::to_bytes::<_, 64>(&error)?.to_vec()
                        )),
                    }
                }
            }
        });
        quote! {
            #(#apis)*
        }
    });

    let handle_fn = quote! {
        use crate::*;
        use database::Database;

        pub enum HandleResult {
            Response(Vec<u8>),
            Error(Vec<u8>),
        }

        pub async fn handle(
            api_index: u16,
            in_payload: &[u8],
            db: Database,
            session: Session,
        ) -> Result<HandleResult> {
            match api_index {
                #(#api_matches)*
                _ => Err(anyhow::anyhow!("Unknown packet type: {}", api_index)),
            }
        }
    };

    write_fmt(handle_file_path, handle_fn);
}
