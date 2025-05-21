// This comment is for Copilot.
// export type NewSystemEvent =
//     | {
//           type: "http-fetch/on-response";
//           fetchId: U32;
//           status: U16;
//           headerCount: U16;
//           headers: {
//               keyByteLength: U16;
//               key: Bytes;
//               valueByteLength: U16;
//               value: Bytes;
//           }[];
//       }
//     | {
//           type: "http-fetch/on-response-body-chunk";
//           fetchId: U32;
//           pooledBufferPtr: U32;
//           written: U32;
//       }
//     | {
//           type: "http-fetch/on-response-body-done";
//           fetchId: U32;
//       }
//     | {
//           type: "http-fetch/on-error";
//           fetchId: U32;
//           messageByteLength: U32;
//           message: Bytes;
//       }
//     | {
//           type: "buffer-pool/request-buffer";
//       }
//     | {
//          type: "insert-js/request-data-buffer";
//          jsId: U32;
//          requestId: U32;
//          bufferLen: U32;
//       }
//     | {
//         type: "insert-js/data";
//         jsId: U32;
//         requestId: U32;
//       }

pub(crate) enum Event {
    HttpFetchOnResponse {
        fetch_id: u32,
        status: u16,
        headers: Vec<(String, String)>,
    },
    HttpFetchOnResponseBodyChunk {
        fetch_id: u32,
        pooled_buffer_ptr: u32,
        written: u32,
    },
    HttpFetchOnResponseBodyDone {
        fetch_id: u32,
    },
    HttpFetchOnError {
        fetch_id: u32,
        message: String,
    },
    BufferPoolRequestBuffer,
    InsertJsRequestDataBuffer {
        js_id: u32,
        request_id: u32,
        buffer_len: u32,
    },
    InsertJsData {
        js_id: u32,
        request_id: u32,
    },
}

pub(crate) fn read(event_buffer: &mut namui_type::RingBuffer) -> Event {
    let message_type = event_buffer.read_u8();
    match message_type {
        1 => {
            let fetch_id = event_buffer.read_u32();
            let status = event_buffer.read_u16();
            let header_count = event_buffer.read_u16();

            let mut headers = Vec::with_capacity(header_count as usize);
            for _ in 0..header_count {
                let key_length = event_buffer.read_u16() as usize;
                let key_bytes = event_buffer.read_bytes(key_length);
                let key = std::str::from_utf8(&key_bytes).unwrap().to_string();

                let value_length = event_buffer.read_u16() as usize;
                let value_bytes = event_buffer.read_bytes(value_length);
                let value = std::str::from_utf8(&value_bytes).unwrap().to_string();

                headers.push((key, value));
            }
            Event::HttpFetchOnResponse {
                fetch_id,
                status,
                headers,
            }
        }
        2 => {
            let fetch_id = event_buffer.read_u32();
            let pooled_buffer_ptr = event_buffer.read_u32();
            let written = event_buffer.read_u32();
            Event::HttpFetchOnResponseBodyChunk {
                fetch_id,
                pooled_buffer_ptr,
                written,
            }
        }
        3 => {
            let fetch_id = event_buffer.read_u32();
            Event::HttpFetchOnResponseBodyDone { fetch_id }
        }
        4 => {
            let fetch_id = event_buffer.read_u32();
            let message_length = event_buffer.read_u32() as usize;
            let message_bytes = event_buffer.read_bytes(message_length);
            let message = std::str::from_utf8(&message_bytes).unwrap().to_string();
            Event::HttpFetchOnError { fetch_id, message }
        }
        5 => Event::BufferPoolRequestBuffer,
        6 => {
            let js_id = event_buffer.read_u32();
            let request_id = event_buffer.read_u32();
            let buffer_len = event_buffer.read_u32();
            Event::InsertJsRequestDataBuffer {
                js_id,
                request_id,
                buffer_len,
            }
        }
        7 => {
            let js_id = event_buffer.read_u32();
            let request_id = event_buffer.read_u32();
            Event::InsertJsData { js_id, request_id }
        }
        _ => unreachable!(),
    }
}
