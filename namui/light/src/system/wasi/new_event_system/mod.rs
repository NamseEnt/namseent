mod event;

use namui_type::RingBuffer;

// TODO: Move other event related code to this system.

pub(crate) async fn init() -> Result<()> {
    spawn_thread();
    Ok(())
}

// # data callback protocol
// [data byte length: u16][message data: ...]
unsafe extern "C" {
    fn _new_event_system_init_thread(event_buffer_ptr: *const u8, event_buffer_len: usize);
    fn _new_event_system_event_poll() -> usize;
    fn _new_event_system_event_commit(byte_length: usize);
}

fn spawn_thread() {
    std::thread::spawn({
        move || {
            let mut event_buffer = RingBuffer::new(4 * 1024 * 1024);

            unsafe {
                _new_event_system_init_thread(event_buffer.ptr(), event_buffer.size());
            }

            loop {
                assert_ne!(unsafe { _new_event_system_event_poll() }, 0);

                let event = event::read(&mut event_buffer);
                unsafe { _new_event_system_event_commit(event_buffer.take_read_count()) };

                match event {
                    event::Event::HttpFetchOnResponse {
                        fetch_id,
                        status,
                        headers,
                    } => crate::system::network::http::http_fetch_on_response(
                        fetch_id, status, headers,
                    ),
                    event::Event::HttpFetchOnResponseBodyChunk {
                        fetch_id,
                        pooled_buffer_ptr,
                        written,
                    } => {
                        let bytes = crate::system::buffer_pool::take_buffer_from_js(
                            pooled_buffer_ptr as *const u8,
                        )
                        .slice(..written as usize);
                        crate::system::network::http::http_fetch_on_response_body_chunk(
                            fetch_id, bytes,
                        );
                    }
                    event::Event::HttpFetchOnResponseBodyDone { fetch_id } => {
                        crate::system::network::http::http_fetch_on_response_body_done(fetch_id);
                    }
                    event::Event::HttpFetchOnError { fetch_id, message } => {
                        crate::system::network::http::http_fetch_on_error(fetch_id, message);
                    }
                    event::Event::BufferPoolRequestBuffer => {
                        crate::system::buffer_pool::send_new_buffer_to_js();
                    }
                    event::Event::InsertJsRequestDataBuffer {
                        js_id,
                        request_id,
                        buffer_len,
                    } => {
                        crate::system::insert_js::on_request_data_buffer(
                            js_id as usize,
                            request_id as usize,
                            buffer_len as usize,
                        );
                    }
                    event::Event::InsertJsData { js_id, request_id } => {
                        crate::system::insert_js::on_data(js_id as usize, request_id as usize);
                    }
                }
            }
        }
    });
}
