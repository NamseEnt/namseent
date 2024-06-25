//! `rpc_macro::define_rpc!` will automatically put `InternalServerError` in the `Error` enum.

rpc_macro::define_rpc! {
    Auth: {
        google_auth: {
            struct Request {
                jwt: String,
            }
            struct Response {
            }
            enum Error {
            }
        },
    },
}
