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
        use $crate::tokio::sync::mpsc::UnboundedSender;
        use $crate::futures::{stream::{TryStreamExt}, Stream};
        use $crate::response_waiter::ResponseWaiter;
        use $crate::serde::{Serialize, Deserialize};
        use $crate::bincode;
        pub use $crate::async_trait;

        pub struct Socket {
            pub sender: UnboundedSender<Vec<u8>>,
            pub response_waiter: ResponseWaiter,
        }
        impl Clone for Socket {
            fn clone(&self) -> Self {
                Self {
                    sender: self
                        .sender
                        .clone(),
                    response_waiter: self
                        .response_waiter
                        .clone(),
                }
            }
        }

        #[derive(Serialize, Deserialize)]
        pub enum RpcPacket {
            Request(RpcRequestPacket),
            Response(RpcResponsePacket),
        }

        #[derive(Serialize, Deserialize)]
        enum RpcApi {
            $(
                #[allow(non_camel_case_types)]
                $request_name,
            )*
        }

        #[derive(Serialize, Deserialize)]
        pub struct RpcRequestPacket {
            id: u64,
            api: RpcApi,
            data: Vec<u8>,
        }

        #[derive(Serialize, Deserialize)]
        pub struct RpcResponsePacket {
            id: u64,
            data: Vec<u8>,
        }

        $(
            #[allow(non_camel_case_types)]
            pub mod $request_name {
                use $crate::serde::{Serialize, Deserialize};
                #[derive(Serialize, Deserialize)]
                pub struct Request {
                    $(pub $request_field: $request_field_type),*
                }
                #[derive(Serialize, Deserialize)]
                pub struct Response {
                    $(pub $response_field: $response_field_type),*
                }
            }
        )*

        #[allow(dead_code)]
        impl Socket {
            pub fn new(sender: $crate::tokio::sync::mpsc::UnboundedSender<Vec<u8>>,
                response_waiter: ResponseWaiter) -> Self {
                Socket {
                    sender,
                    response_waiter,
                }
            }
            async fn send<'de, TRequest, TResponse>(
                &mut self,
                request: TRequest,
                api: RpcApi,
            ) -> Result<TResponse, String>
            where
                TRequest: Serialize,
                TResponse: $crate::serde::de::DeserializeOwned,
            {
                static mut ID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                let id = unsafe { ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) };

                let request_packet = RpcPacket::Request(RpcRequestPacket {
                    id,
                    api,
                    data: bincode::serialize(&request).unwrap(),
                });

                let notify = self
                    .response_waiter
                    .ready_to_wait(id);

                let data = bincode::serialize(&request_packet).unwrap();
                self.sender
                    .send(data)
                    .unwrap();

                let response_data = self
                    .response_waiter
                    .wait(id, notify)
                    .await?;

                let response: Result<TResponse, String> = bincode::deserialize(&response_data).unwrap();

                Ok(response.unwrap())
            }

            $(
                pub async fn $request_name(&mut self,
                    request: $request_name::Request
                ) -> Result<$request_name::Response, String> {
                    self.send(request, RpcApi::$request_name).await
                }
            )*
        }

        #[async_trait::async_trait]
        pub trait RpcHandle {
            $(
                async fn $request_name(&mut self,
                    request: $request_name::Request
                ) -> Result<$request_name::Response, String>;
            )*
        }
        #[allow(dead_code)]
        pub async fn loop_receiving<'a, TRpcHandle, TStream>(
            sender: UnboundedSender<Vec<u8>>,
            stream: TStream,
            handler: TRpcHandle,
            response_waiter: ResponseWaiter,
        ) -> Result<(), String> where
            TRpcHandle: RpcHandle + Sized + Clone + std::marker::Unpin,
            TStream: Stream<Item = Result<Vec<u8>, String>> + Sized + std::marker::Unpin,
        {
            stream
                .try_for_each_concurrent(None, |packet_buffer| {
                    let mut handler = Box::new(handler.clone());
                    let sender = Box::new(sender.clone());
                    let response_waiter = Box::new(response_waiter.clone());
                    async move {
                        if packet_buffer.len() == 0 {
                            return Err("done".to_string());
                        };

                        let deserialize_result = bincode::deserialize::<RpcPacket>(&packet_buffer);
                        match deserialize_result {
                            Ok(packet) => match packet {
                                RpcPacket::Request(request_packet) => match request_packet.api {
                                    $(
                                        RpcApi::$request_name => {
                                            let request =
                                                bincode::deserialize::<$request_name::Request>(
                                                    &request_packet.data,
                                                )
                                                .unwrap();
                                            let response = handler
                                                .$request_name(request)
                                                .await;
                                            let response_packet = RpcPacket::Response(RpcResponsePacket {
                                                id: request_packet.id,
                                                data: bincode::serialize(&response).unwrap(),
                                            });
                                            let response_packet_buffer =
                                                bincode::serialize(&response_packet).unwrap();
                                            sender
                                                .send(response_packet_buffer)
                                                .unwrap();

                                            Ok(())
                                        }
                                    )*
                                },
                                RpcPacket::Response(response) => {
                                    response_waiter.on_response(response.id, response.data);

                                    Ok(())
                                }
                            },
                            Err(error) => {
                                eprintln!("{}", error);
                                Err(error.to_string())
                            }
                        }
                    }
                })
                .await
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use async_trait::async_trait;
    use futures::{future::join, join, StreamExt};
    use tokio;
    use tokio_stream::wrappers::UnboundedReceiverStream;

    #[derive(Deserialize, Serialize)]
    pub struct DirectoryEntry {}

    def_rpc! {
        ls({ path: String, }) -> {
            directory_entries: Vec<super::DirectoryEntry>,
        },
    }

    mod test_zero_request_param {
        def_rpc! {
            zero_request_param({ }) -> {
                directory_entries: Vec<String>,
            },
        }
    }

    #[tokio::test]
    async fn test_socket() {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        let response_waiter = ResponseWaiter::new();
        let mut socket = Socket::new(sender.clone(), response_waiter.clone());

        #[derive(Clone)]
        struct RpcHandler {
            number: i32,
        }

        #[async_trait]
        impl RpcHandle for RpcHandler {
            async fn ls(&mut self, request: ls::Request) -> Result<ls::Response, String> {
                Ok(ls::Response {
                    directory_entries: vec![DirectoryEntry {}],
                })
            }
        }
        let rpc_handler = RpcHandler {
            number: 0,
        };

        let stream = UnboundedReceiverStream::new(receiver);
        let stream = stream.map(|item| Ok(item));

        let cloned_sender = sender.clone();
        let receiving = async move {
            let result =
                loop_receiving(cloned_sender, stream, rpc_handler, response_waiter.clone()).await;
        };

        let cloned_sender = sender.clone();
        let sending = async move {
            let response = socket
                .ls(ls::Request {
                    path: "abc".to_string(),
                })
                .await;
            assert_eq!(
                response
                    .unwrap()
                    .directory_entries
                    .len(),
                1
            );
            cloned_sender
                .send(vec![])
                .unwrap();
        };
        join!(receiving, sending);
    }
}
