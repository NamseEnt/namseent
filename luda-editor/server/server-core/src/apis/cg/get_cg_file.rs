use crate::documents::*;
use rpc::get_cg_file::{Error, Request, Response};

pub async fn get_cg_file(
    _session: Option<SessionDocument>,
    Request { project_id, cg_id }: Request,
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
    .map_err(|err| Error::Unknown(err.to_string()))?;

    Ok(Response { cg_file })
}
