export type Exports = {
    wasi_thread_start: (tid: number, startArgPtr: number) => void;
    malloc: (size: number) => number;
    free: (ptr: number) => void;
};
