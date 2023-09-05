use crate::documents::*;
use rpc::get_cg_file::{Error, Request, Response};

pub async fn get_cg_file(
    _session: Option<SessionDocument>,
    rpc::get_cg_file::Request { project_id, cg_id }: rpc::get_cg_file::Request,
) -> rpc::get_cg_file::Result {
    let CgDocument {
        cg_file,
        project_id: _,
        cg_id: _,
    } = CgDocumentGet {
        pk_project_id: project_id,
        sk_cg_id: cg_id,
    }
    .run()
    .await
    .map_err(|err| rpc::get_cg_file::Error::Unknown(err.to_string()))?;

    Ok(rpc::get_cg_file::Response { cg_file })
}
