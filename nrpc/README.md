# Features

- Bidirectional
- Request and response
- Fire and forget
- Trait Generator Macro

# Layers

- Interface Layer

  - Expose methods to request or handler to response

- Request Layer
- Handler Layer

- Recall Layer
  - Remember request id to pass response
- Transport Layer
  - Data packet transportation
  - Send and Receive

┌─────────┬─────────┐
│ Request │ Handler │
├─────────┼─────────┤
│ Recall │ │
├─────────┴─────────┤
│ Transport │
└───────────────────┘

Api

- async fn $rpc_name($request) -> Result<$response, String>
- trait Handler { fn $rpc_name($request) -> $response }
- trait PacketSender { fn send(packet: &[u8]) }
-

rx: packet -> handler

```rs
pub struct Socket {
  tx,
  request_response_connector,
}
impl Socket {
  pub fn new(tx) -> Self {

  }
  pub async fn api(...)... {
    tx.send(request_packet);
    let response = request_response_connector.wait(request_packet.id).await;
    response
  }
}
impl Clone for Socket {

}

pub async fn loop_receiving(tx, rx, handler, request_response_connector) {
  loop {
    let packet_buffer = rx.next().await;
    let packet = serde::deserialize::<RpcPacket>(packet_buffer);
    match packet {
      Request(request_packet) {
        let request = deserialize(request_packet);
        let response = handler.handle(request);
        let response_packet = serialize(response);
        tx.send(response_packet);
      }
      Response(response_packet) {
        // notify
        let response = deserialize(response_packet);
        let notify = request_response_connector.notify(&response.id, response.data);
      }
    }
  }
}
```

# Packet

```rs
enum RpcPacket {
  Request(RpcRequestPacket),
  Response(RpcResponsePacket),
}

enum RpcApi {
  ...
}

struct RpcRequestPacket {
  id: u64,
  api: RpcApi,
  data: &[u8]
}

struct RpcResponsePacket {
  id: u64,
  data: &[u8]
}
```
