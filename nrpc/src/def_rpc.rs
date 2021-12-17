#[macro_export]
macro_rules! def_rpc {
    {
        $(
            $request_name:ident({
                $($request_field:ident: $request_field_type:ty),* $(,)?
            }) -> Result<{
                $($response_field:ident: $response_field_type:ty),* $(,)?
            }, $response_err_type:ty> $(,)?
        ),*
    } => {
        pub struct Socket {}

        $(
            pub mod $request_name {
                pub struct Request {
                    $(pub $request_field: $request_field_type),*
                }
                pub struct Response {
                    $(pub $response_field: $response_field_type),*
                }
            }
        )*

        impl Socket {
            // pub fn new(recall_layer: RecallLayer) -> Self {
            //     Self {
            //        recall_layer,
            //     }
            // }
            $(
                pub fn $request_name(&self,
                    request: $request_name::Request
                ) -> Result<$request_name::Response, $response_err_type> {
                    request;
                    todo!()
                }
            )*
        }

        pub trait Handler {
            $(
                fn $request_name(&self,
                    request: $request_name::Request
                ) -> Result<$request_name::Response, $response_err_type>;
            )*
        }
    };
}

#[cfg(test)]
mod tests {
    pub struct DirectoryEntry {}

    def_rpc! {
        ls({ path: String, }) -> Result<{
            directory_entries: Vec<super::DirectoryEntry>,
        }, String>,

        get_file({
            path: String
        }) -> Result<{
        data: Vec<u8>
        }, String>,
    }

    #[test]
    fn test_socket() {
        let socket = Socket {};
        let response = socket.ls(ls::Request {
            path: "".to_string(),
        });
        response.map(|response| response.directory_entries);

        let response = socket.get_file(get_file::Request {
            path: "".to_string(),
        });
        response.map(|response| response.data);
    }

    #[test]
    fn test_handler() {
        struct HandlerContext {
            number: i32,
        }
        impl Handler for HandlerContext {
            fn ls(&self, request: ls::Request) -> Result<ls::Response, String> {
                self.number;
                request.path;
                todo!()
            }
            fn get_file(&self, request: get_file::Request) -> Result<get_file::Response, String> {
                request.path;
                todo!()
            }
        }
        let handler = HandlerContext {
            number: 0,
        };
    }
}
