#[derive(crate::serde::Serialize)]
struct Test {}

#[macro_export]
macro_rules! def_rpc {
    {
        $(
            $request_name:ident({
                $($request_field:ident: $request_field_type:ty),* $(,)?
            }) -> {
                $($response_field:ident: $response_field_type:ty),* $(,)?
            } $(,)?
        ),*
    } => {
        pub struct Socket {
            recall_layer: $crate::recall_layer::RecallLayer,
        }

        $(
            pub mod $request_name {
                #[derive($crate::serde::Serialize, $crate::serde::Deserialize)]
                pub struct Request {
                    $(pub $request_field: $request_field_type),*
                }
                #[derive($crate::serde::Serialize, $crate::serde::Deserialize)]
                pub struct Response {
                    $(pub $response_field: $response_field_type),*
                }
            }
        )*

        impl Socket {
            pub fn new(
                mut transport_layer: impl $crate::transport_layer::TransportLayer + 'static) -> Self {
                Self {
                   recall_layer: $crate::recall_layer::RecallLayer::new(transport_layer),
                }
            }
            $(
                pub async fn $request_name(&mut self,
                    request: $request_name::Request
                ) -> Result<$request_name::Response, String> {
                    let request_data = $crate::bincode::serialize(&request).unwrap();
                    let response_data = self.recall_layer.send(request_data).await;
                    if let Err(err) = response_data {
                        return Err(err);
                    }
                    let response_data = response_data.unwrap();
                    let response: Result<$request_name::Response, String> =
                        $crate::bincode::deserialize(&response_data[..])
                        .map_err(|e| e.to_string());
                    response
                }
            )*
        }

        pub trait RpcHandle {
            $(
                fn $request_name(&self,
                    request: $request_name::Request
                ) -> Result<$request_name::Response, String>;
            )*
        }
    };
}

fn test() {}

#[cfg(test)]
mod tests {
    use crate::serde::{Deserialize, Serialize};
    use crate::{recall_layer::RecallLayerReceiver, TransportLayer};
    use async_trait::async_trait;

    #[derive(Deserialize, Serialize)]
    pub struct DirectoryEntry {}

    def_rpc! {
        ls({ path: String, }) -> {
            directory_entries: Vec<super::DirectoryEntry>,
        },

        get_file({
            path: String
        }) -> {
        data: Vec<u8>
        },
    }

    #[tokio::test]
    async fn test_socket() {
        struct TestTransportLayer {
            recall_layer_receiver: Option<RecallLayerReceiver>,
        }
        #[async_trait]
        impl TransportLayer for TestTransportLayer {
            async fn send(&mut self, packet: Vec<u8>) -> Result<(), String> {
                let request = ls::Request {
                    path: "abc".to_string(),
                };
                let mut request_packet = vec![0, 0, 0, 0, 0, 0, 0, 0];
                request_packet.extend_from_slice(&bincode::serialize(&request).unwrap());

                assert_eq!(packet, request_packet);

                let response = ls::Response {
                    directory_entries: vec![DirectoryEntry {}],
                };
                let mut response_packet = vec![0, 0, 0, 0, 0, 0, 0, 0];
                response_packet.extend_from_slice(&bincode::serialize(&response).unwrap());
                self.recall_layer_receiver
                    .as_ref()
                    .unwrap()
                    .on_received(response_packet);
                Ok(())
            }

            fn set_recall_layer_receiver(&mut self, recall_layer_receiver: RecallLayerReceiver) {
                self.recall_layer_receiver = Some(recall_layer_receiver);
            }
        }

        let transport_layer = TestTransportLayer {
            recall_layer_receiver: None,
        };

        let mut socket = Socket::new(transport_layer);
        let response = socket
            .ls(ls::Request {
                path: "abc".to_string(),
            })
            .await;
        let _ = response.map(|response| response.directory_entries);
    }

    #[test]
    fn test_handler() {
        struct HandlerContext {
            number: i32,
        }
        impl RpcHandle for HandlerContext {
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
        let _ = HandlerContext {
            number: 0,
        };
    }
}
