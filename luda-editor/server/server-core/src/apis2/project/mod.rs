pub mod documents;
mod impls;

use crate::session::SessionDocument;
use documents::*;
use futures::future::try_join_all;

#[derive(Debug)]
pub struct ProjectService {}

impl ProjectService {
    pub fn new() -> Self {
        ProjectService {}
    }
}

impl rpc::ProjectService<SessionDocument> for ProjectService {
    fn create_project<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::create_project::Request,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<
                    Output = Result<rpc::create_project::Response, rpc::create_project::Error>,
                >
                + Send,
        >,
    > {
    }

    fn list_editable_projects<'a>(
        &'a self,
        session: Option<SessionDocument>,
        _req: rpc::list_editable_projects::Request,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<
                    Output = Result<
                        rpc::list_editable_projects::Response,
                        rpc::list_editable_projects::Error,
                    >,
                >
                + Send,
        >,
    > {
        Box::pin(async move {})
    }

    fn edit_user_acl<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::edit_user_acl::Request,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<
                    Output = Result<rpc::edit_user_acl::Response, rpc::edit_user_acl::Error>,
                >
                + Send,
        >,
    > {
        Box::pin(async move {})
    }

    fn update_server_project_shared_data<'a>(
        &'a self,
        session: Option<SessionDocument>,
        req: rpc::update_server_project_shared_data::Request,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<Output = rpc::update_server_project_shared_data::Result>
                + Send,
        >,
    > {
        Box::pin(async move {})
    }

    fn update_client_project_shared_data<'a>(
        &'a self,
        _session: Option<SessionDocument>,
        req: rpc::update_client_project_shared_data::Request,
    ) -> std::pin::Pin<
        Box<
            dyn 'a
                + std::future::Future<Output = rpc::update_client_project_shared_data::Result>
                + Send,
        >,
    > {
        Box::pin(async move {})
    }
}
