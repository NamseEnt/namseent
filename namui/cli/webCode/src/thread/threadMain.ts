import {
    ConsoleStdout,
    File,
    OpenFile,
    WASI,
    type Fd,
} from "@bjorn3/browser_wasi_shim";
import { patchWasi } from "../patchWasi";
import ThreadWorker from "./ThreadWorker?worker";

export type ThreadStartSupplies = {
    nextTid: SharedArrayBuffer; // 4 bytes
    memory: WebAssembly.Memory;
    module: WebAssembly.Module;
    tabId: string;
} & (
    | {
          type: "main";
      }
    | {
          type: "thread";
          startArgPtr: number;
          tid: number;
      }
);

export async function threadMain(supplies: ThreadStartSupplies) {
    const { memory, module, tabId } = supplies;

    const broadcastChannel = new BroadcastChannel(tabId);

    const fd: Fd[] = [
        new OpenFile(new File([])), // stdin
        ConsoleStdout.lineBuffered((msg) => console.log(msg)),
        ConsoleStdout.lineBuffered((msg) => console.log(`[stderr] ${msg}`)),
    ];

    const env = ["RUST_BACKTRACE=full"];

    const wasi = new WASI([], env, fd);
    patchWasi(wasi);

    let nextFetchId = 0;

    const instance = await WebAssembly.instantiate(module, {
        env: {
            memory,
            _hardware_concurrency: () => navigator.hardwareConcurrency,
            _http_fetch_start: (
                uri_ptr: number,
                uri_byte_len: number,
                method_ptr: number,
                method_byte_len: number,
                headers_ptr: number,
                headers_byte_len: number,
                _body_ptr: number,
                _body_byte_len: number,
            ): number => {
                // SharedArrayBuffer에서 복사하는 헬퍼 함수
                const copyAndDecode = (ptr: number, len: number): string => {
                    const source = new Uint8Array(memory.buffer, ptr, len);
                    const copied = new Uint8Array(len);
                    copied.set(source);
                    return new TextDecoder().decode(copied);
                };

                // URI 읽기
                const uri = copyAndDecode(uri_ptr, uri_byte_len);

                // Method 읽기
                const method = copyAndDecode(method_ptr, method_byte_len);

                // Headers 읽기 (key-len, key-bytes, value-len, value-bytes 형식)
                const headers: Record<string, string> = {};
                const headersView = new DataView(
                    memory.buffer,
                    headers_ptr,
                    headers_byte_len,
                );
                let offset = 0;
                while (offset < headers_byte_len) {
                    // key length
                    const keyLen = headersView.getUint32(offset, true);
                    offset += 4;
                    // key bytes
                    const key = copyAndDecode(headers_ptr + offset, keyLen);
                    offset += keyLen;
                    // value length
                    const valueLen = headersView.getUint32(offset, true);
                    offset += 4;
                    // value bytes
                    const value = copyAndDecode(headers_ptr + offset, valueLen);
                    offset += valueLen;

                    headers[key] = value;
                }

                // TODO: body 처리
                // const bodyBytes = new Uint8Array(memory.buffer, body_ptr, body_byte_len);

                const fetchId = nextFetchId++;
                broadcastChannel.postMessage({
                    tabId,
                    type: "fetch",
                    fetchId,
                    uri,
                    method,
                    headers,
                });

                return fetchId;
            },
            _http_fetch_finish_request_body_stream: () => {},
            _http_fetch_error_on_rust_side: () => {},
        },
        wasi_snapshot_preview1: wasi.wasiImport,
        wasi: {
            "thread-spawn": (startArgPtr: number) => {
                const tid = Atomics.add(
                    new Uint32Array(supplies.nextTid),
                    0,
                    1,
                );
                const worker = new ThreadWorker();
                worker.postMessage({
                    ...supplies,
                    type: "thread",
                    startArgPtr,
                    tid,
                } satisfies ThreadStartSupplies);

                return tid;
            },
        },
    });

    const exports = instance.exports as any;

    switch (supplies.type) {
        case "main":
            wasi.start(instance as any);
            break;
        case "thread":
            wasi.initialize(instance as any);
            exports.wasi_thread_start(supplies.tid, supplies.startArgPtr);
            break;
    }

    return instance;
}
