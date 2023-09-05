use crate::documents::*;
use rpc::list_cg_files::{Error, Request, Response};

pub async fn list_cg_files(
    _session: Option<SessionDocument>,
    rpc::list_cg_files::Request { project_id }: rpc::list_cg_files::Request,
) -> rpc::list_cg_files::Result {
    let cg_files = CgDocumentQuery {
        pk_project_id: project_id,
        last_sk: None, // TODO
    }
    .run()
    .await
    .map_err(|err| rpc::list_cg_files::Error::Unknown(err.to_string()))?
    .documents
    .into_iter()
    .map(
        |CgDocument {
             project_id: _,
             cg_id: _,
             cg_file,
         }| cg_file,
    )
    .collect();

    Ok(rpc::list_cg_files::Response { cg_files })
}
