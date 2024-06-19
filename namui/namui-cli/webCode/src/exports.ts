export type Exports = {
    wasi_thread_start: (tid: number, startArgPtr: number) => void;
    _malloc: (size: number) => number;
    _free: (ptr: number) => void;

    on_web_socket_open: (id: number) => void;
    on_web_socket_close: (id: number) => void;
    web_socket_message_alloc: (data_len: number) => number;
    on_web_socket_message: (id: number, data_ptr: number) => void;
};
