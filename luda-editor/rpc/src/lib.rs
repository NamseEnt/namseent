mod define_rpc;

pub use base64;

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
                pub session_id: String,
            }
            Error {
                AlreadyLoggedIn,
                Unknown(String),
            }
        },
    },
    SequenceService: {
        list_project_sequences: {
            pub struct Request {
                pub project_id: String,
            }
            pub struct Response {
                pub sequence_name_and_ids: Vec<SequenceNameAndId>,
            }
            pub struct SequenceNameAndId {
                pub name: String,
                pub id: String,
            }
            Error {
                Unknown(String),
            }
        },
        create_sequence: {
            pub struct Request {
                pub project_id: String,
                pub name: String,
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
                pub sequence_id: String,
                pub client_state_vector_base64: String,
                pub e_tag: Option<String>,
            }
            pub enum Response {
                Modified {
                    server_state_vector_base64: String,
                    yrs_update_v2_for_client_base64: String,
                    e_tag: String,
                },
                NotModified,
            }
            Error {
                ServerSequenceNotExists,
                Unknown(String),
            }
        },
        update_server_sequence: {
            pub struct Request {
                pub sequence_id: String,
                pub client_state_vector_base64: String,
                pub yrs_update_v2_for_server_base64: String,
            }
            pub struct Response {
                pub server_state_vector_base64: String,
                pub yrs_update_v2_for_client_base64: String,
                pub e_tag: String,
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
    },
    ResourceService: {
        get_resource: {
            pub struct Request {
                pub resource_id: String,
            }
            pub struct Response {
                pub base64: String,
            }
            Error {
                NotFound,
                Unknown(String),
            }
        },
        list_resources: {
            pub struct Request {
                pub start_after: Option<String>,
            }
            pub struct Response {
                pub resource_keys: Vec<String>,
            }
            Error {
                Unknown(String),
            }
        },
        put_resource: {
            pub struct Request {
                pub resource_id: String,
                pub base64: String,
            }
            pub struct Response {}
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
                pub id: String,
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
                pub project_id: String,
                pub user_id: String,
                pub permission: Option<crate::types::ProjectAclUserPermission>,
            }
            pub struct Response {}
            Error {
                Unauthorized,
                CannotSetOwnerPermission,
                Unknown(String),
            }
        },
    },
}
