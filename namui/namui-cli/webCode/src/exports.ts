export type Exports = {
    memory: WebAssembly.Memory;
    wasi_thread_start: (tid: number, startArgPtr: number) => void;
    malloc: (size: number) => number;
    free: (ptr: number) => void;
    _on_event: (ptr: number, len: number) => void;
};
