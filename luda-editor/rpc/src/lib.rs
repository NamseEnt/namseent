pub mod data;
mod define_rpc;

pub use define_rpc::RpcFuture;
pub use revert_json_patch as json_patch;
pub use uuid::{uuid, Uuid};

#[macro_export]
macro_rules! simple_error_impl {
    ($error_struct: ident) => {
        impl std::fmt::Display for $error_struct {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
        impl std::error::Error for $error_struct {}
    };
}

pub mod types {
    #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
    pub enum ProjectAclUserPermission {
        Editor,
    }
}

define_rpc::define_rpc! {
    AuthService: {
        exchange_google_auth_code_to_access_token: {
            pub struct Request {
                pub code: String,
            }
            pub struct Response {}
            Error {
                Unknown(String)
            }
        },
        log_in_with_github_oauth_code: {
            pub struct Request {
                pub code: String,
            }
            pub struct Response {
                pub session_id: uuid::Uuid,
            }
            Error {
                AlreadyLoggedIn,
                Unknown(String),
            }
        },
        validate_session: {
            pub struct Request {}
            pub struct Response {}
            Error {
                InvalidSession,
                Unknown(String),
            }
        },
    },
    SequenceService: {
        list_project_sequences: {
            pub struct Request {
                pub project_id: uuid::Uuid,
            }
            pub struct Response {
                pub sequence_name_and_ids: Vec<SequenceNameAndId>,
            }
            pub struct SequenceNameAndId {
                pub name: String,
                pub id: uuid::Uuid,
            }
            Error {
                Unknown(String),
            }
        },
        create_sequence: {
            pub struct Request {
                pub project_id: uuid::Uuid,
                pub name: String,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        update_server_sequence: {
            pub struct Request {
                pub sequence_id: uuid::Uuid,
                pub patch: revert_json_patch::Patch,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        update_client_sequence: {
            pub struct Request {
                pub sequence_id: uuid::Uuid,
                pub sequence_json: serde_json::Value,
            }
            pub struct Response {
                pub patch: revert_json_patch::Patch,
            }
            Error {
                Unknown(String),
            }
        },
        get_sequence_and_project_shared_data: {
            pub struct Request {
                pub sequence_id: uuid::Uuid,
            }
            pub struct Response {
                pub sequence_json: String,
                pub project_shared_data_json: String,
            }
            Error {
                Unknown(String),
            }
        },
        delete_sequence: {
            pub struct Request {
                pub sequence_id: uuid::Uuid,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        rename_sequence: {
            pub struct Request {
                pub sequence_id: uuid::Uuid,
                pub new_name: String,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
    },
    ImageService: {
        put_image_meta_data: {
            pub struct Request {
                pub project_id: uuid::Uuid,
                pub image_id: uuid::Uuid,
                pub labels: Vec<crate::data::Label>,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        prepare_upload_image: {
            pub struct Request {
                pub project_id: uuid::Uuid,
                pub image_id: uuid::Uuid,
            }
            pub struct Response {
                pub upload_url: String,
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        list_images: {
            pub struct Request {
                pub project_id: uuid::Uuid,
            }
            pub struct Response {
                pub images: Vec<crate::data::ImageWithLabels>
            }
            Error {
                Unknown(String),
            }
        },
    },
    ProjectService: {
        create_project: {
            pub struct Request {
                pub name: String,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        list_editable_projects: {
            pub struct EditableProject {
                pub id: uuid::Uuid,
                pub name: String,
            }
            pub struct Request {
                pub start_after: Option<String>,
            }
            pub struct Response {
                pub projects: Vec<EditableProject>,
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        edit_user_acl: {
            pub struct Request {
                pub project_id: uuid::Uuid,
                pub user_id: uuid::Uuid,
                pub permission: Option<crate::types::ProjectAclUserPermission>,
            }
            pub struct Response {}
            Error {
                Unauthorized,
                CannotSetOwnerPermission,
                Unknown(String),
            }
        },
        update_server_project_shared_data: {
            pub struct Request {
                pub project_id: uuid::Uuid,
                pub patch: revert_json_patch::Patch,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        update_client_project_shared_data: {
            pub struct Request {
                pub project_id: uuid::Uuid,
                pub project_shared_data_json: serde_json::Value,
            }
            pub struct Response {
                pub patch: revert_json_patch::Patch,
            }
            Error {
                Unknown(String),
            }
        },
    },
}
