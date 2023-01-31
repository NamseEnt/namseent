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
        // TODO: Prevent duplicate project names.
        Box::pin(async move {
            if session.is_none() {
                return Err(rpc::create_project::Error::Unauthorized);
            }
            let session = session.unwrap();

            let project_id = rpc::Uuid::new_v4();
            let owner_id = session.user_id;

            let project_document = ProjectDocument {
                id: project_id,
                owner_id: owner_id,
                name: req.name,
                shared_data_json: serde_json::to_string(&rpc::data::ProjectSharedData::new(
                    project_id,
                ))
                .unwrap(),
            };

            let owner_project_document = OwnerProjectDocument {
                owner_id,
                project_id,
            };

            let result = crate::dynamo_db()
                .transact()
                .create_item(project_document)
                .create_item(owner_project_document)
                .send()
                .await;
            match result {
                Ok(_) => Ok(rpc::create_project::Response {}),
                Err(error) => Err(rpc::create_project::Error::Unknown(error.to_string())),
            }
        })
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
        Box::pin(async move {
            if session.is_none() {
                return Err(rpc::list_editable_projects::Error::Unauthorized);
            }
            let session = session.unwrap();
            let owner_project_documents = OwnerProjectDocumentQuery {
                pk_owner_id: session.user_id,
            }
            .run()
            .await;
            if let Err(error) = owner_project_documents {
                return Err(rpc::list_editable_projects::Error::Unknown(
                    error.to_string(),
                ));
            }
            let owner_project_documents = owner_project_documents.unwrap();

            let editable_projects = try_join_all(owner_project_documents.into_iter().map(
                |owner_project_document| async move {
                    match (ProjectDocumentGet {
                        pk_id: owner_project_document.project_id,
                    })
                    .run()
                    .await
                    {
                        Ok(project) => Ok(rpc::list_editable_projects::EditableProject {
                            id: owner_project_document.project_id,
                            name: project.name,
                        }),
                        Err(error) => Err(rpc::list_editable_projects::Error::Unknown(
                            error.to_string(),
                        )),
                    }
                },
            ))
            .await;
            if let Err(error) = editable_projects {
                return Err(rpc::list_editable_projects::Error::Unknown(
                    error.to_string(),
                ));
            }
            let editable_projects = editable_projects.unwrap();

            Ok(rpc::list_editable_projects::Response {
                projects: editable_projects,
            })
        })
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
        Box::pin(async move {
            if session.is_none() {
                return Err(rpc::edit_user_acl::Error::Unauthorized);
            }
            let session = session.unwrap();

            let project = ProjectDocumentGet {
                pk_id: req.project_id,
            }
            .run()
            .await;
            if let Err(error) = project {
                return Err(rpc::edit_user_acl::Error::Unknown(error.to_string()));
            }
            let project = project.unwrap();

            if req.user_id == project.owner_id {
                return Err(rpc::edit_user_acl::Error::CannotSetOwnerPermission);
            }

            let is_session_has_permission = project.owner_id == session.user_id;

            if !is_session_has_permission {
                return Err(rpc::edit_user_acl::Error::Unauthorized);
            }

            match req.permission {
                Some(permission) => crate::dynamo_db()
                    .transact()
                    .update_item(
                        &project.id,
                        Some(session.user_id),
                        move |mut document: UserInProjectAclDocument| async move {
                            document.permission = permission;
                            Ok(document)
                        },
                    )
                    .update_item(
                        &session.user_id,
                        Some(project.id),
                        move |mut document: ProjectAclUserInDocument| async move {
                            document.permission = permission;
                            Ok(document)
                        },
                    )
                    .send()
                    .await
                    .map(|_| rpc::edit_user_acl::Response {})
                    .map_err(|error| rpc::edit_user_acl::Error::Unknown(error.to_string())),
                None => crate::dynamo_db()
                    .transact()
                    .command(UserInProjectAclDocumentDelete {
                        pk_project_id: project.id,
                        sk_user_id: session.user_id,
                    })
                    .command(ProjectAclUserInDocumentDelete {
                        pk_user_id: session.user_id,
                        sk_project_id: project.id,
                    })
                    .send()
                    .await
                    .map(|_| rpc::edit_user_acl::Response {})
                    .map_err(|error| rpc::edit_user_acl::Error::Unknown(error.to_string())),
            }
        })
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
        Box::pin(async move {
            if session.is_none() {
                return Err(rpc::update_server_project_shared_data::Error::Unauthorized);
            }
            let session = session.unwrap();

            let is_project_editor = self
                .is_project_editor(session.user_id, req.project_id)
                .await
                .map_err(|error| {
                    rpc::update_server_project_shared_data::Error::Unknown(error.to_string())
                })?;

            if !is_project_editor {
                return Err(rpc::update_server_project_shared_data::Error::Unauthorized);
            }

            crate::dynamo_db()
                .update_item::<_, rpc::update_server_project_shared_data::Error, _>(
                    &req.project_id,
                    Option::<String>::None,
                    |mut project: ProjectDocument| async {
                        let mut project_shared_data_json =
                            serde_json::from_str::<serde_json::Value>(&project.shared_data_json)
                                .map_err(|err| {
                                    rpc::update_server_project_shared_data::Error::Unknown(
                                        err.to_string(),
                                    )
                                })?;
                        rpc::json_patch::patch(&mut project_shared_data_json, &req.patch).map_err(
                            |err| {
                                rpc::update_server_project_shared_data::Error::Unknown(
                                    err.to_string(),
                                )
                            },
                        )?;

                        project.shared_data_json = serde_json::to_string(&project_shared_data_json)
                            .map_err(|err| {
                                rpc::update_server_project_shared_data::Error::Unknown(
                                    err.to_string(),
                                )
                            })?;
                        Ok(project)
                    },
                )
                .await
                .map_err(|error| match error {
                    crate::storage::dynamo_db::UpdateItemError::Canceled(error) => error,
                    crate::storage::dynamo_db::UpdateItemError::NotFound
                    | crate::storage::dynamo_db::UpdateItemError::SerializationFailed(_)
                    | crate::storage::dynamo_db::UpdateItemError::Conflict
                    | crate::storage::dynamo_db::UpdateItemError::Unknown(_) => {
                        rpc::update_server_project_shared_data::Error::Unknown(error.to_string())
                    }
                })?;

            Ok(rpc::update_server_project_shared_data::Response {})
        })
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
        Box::pin(async move {
            let project = ProjectDocumentGet {
                pk_id: req.project_id,
            }
            .run()
            .await
            .map_err(|error| {
                rpc::update_client_project_shared_data::Error::Unknown(error.to_string())
            })?;

            let project_shared_data_json =
                serde_json::from_str::<serde_json::Value>(&project.shared_data_json).map_err(
                    |err| rpc::update_client_project_shared_data::Error::Unknown(err.to_string()),
                )?;
            let patch =
                rpc::json_patch::diff(&req.project_shared_data_json, &project_shared_data_json);

            Ok(rpc::update_client_project_shared_data::Response { patch })
        })
    }
}
