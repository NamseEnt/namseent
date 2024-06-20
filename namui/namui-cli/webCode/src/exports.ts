export type Exports = {
    wasi_thread_start: (tid: number, startArgPtr: number) => void;
    _malloc: (size: number) => number;
    _free: (ptr: number) => void;
};
