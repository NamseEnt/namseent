import { BufferPoolHandleOnMainThread } from "../bufferPool";
import { sendMessageToMainThread } from "../interWorkerProtocol";
import { NoStreamHttpFetchHandle } from "./noStream";
import { YesStreamHttpFetchHandle } from "./yesStream";
import {
    NewEventSystemHandleOnMainThread,
    NewSystemEvent,
} from "../newEventSystem";

export function httpFetchImports({ memory }: { memory: WebAssembly.Memory }) {
    return {
        _http_fetch_init(
            urlPtr: number,
            urlLen: number,
            methodPtr: number,
            methodLen: number,
        ): number {
            const urlBuffer = new Uint8Array(urlLen);
            urlBuffer.set(new Uint8Array(memory.buffer, urlPtr, urlLen));
            const url = new TextDecoder().decode(urlBuffer);

            const methodBuffer = new Uint8Array(methodLen);
            methodBuffer.set(
                new Uint8Array(memory.buffer, methodPtr, methodLen),
            );
            const method = new TextDecoder().decode(methodBuffer);

            const idBuffer = new SharedArrayBuffer(4);
            sendMessageToMainThread({
                type: "new-http-fetch",
                url,
                method,
                idBuffer,
            });

            Atomics.wait(new Int32Array(idBuffer), 0, 0);
            const id = new Uint32Array(idBuffer)[0];
            return id;
        },
        _http_fetch_set_header(
            fetchId: number,
            keyPtr: number,
            keyLen: number,
            valuePtr: number,
            valueLen: number,
        ) {
            const keyBuffer = new Uint8Array(keyLen);
            keyBuffer.set(new Uint8Array(memory.buffer, keyPtr, keyLen));
            const key = new TextDecoder().decode(keyBuffer);

            const valueBuffer = new Uint8Array(valueLen);
            valueBuffer.set(new Uint8Array(memory.buffer, valuePtr, valueLen));
            const value = new TextDecoder().decode(valueBuffer);

            sendMessageToMainThread({
                type: "http-fetch-set-header",
                fetchId,
                key,
                value,
            });
        },
        _http_fetch_start(fetchId: number) {
            sendMessageToMainThread({
                type: "http-fetch-start",
                fetchId,
            });
        },
        _http_fetch_push_request_body_chunk(
            fetchId: number,
            dataPtr: number,
            dataLen: number,
        ) {
            // TODO: Reduce this copy, just passing ptr and tracking lifetime
            const data = new Uint8Array(dataLen);
            data.set(new Uint8Array(memory.buffer, dataPtr, dataLen));
            sendMessageToMainThread(
                {
                    type: "http-fetch-push-request-body-chunk",
                    fetchId,
                    data: data.buffer,
                },
                [data.buffer],
            );
        },
        _http_fetch_finish_request_body_stream(fetchId: number) {
            sendMessageToMainThread({
                type: "http-fetch-finish-request-body-stream",
                fetchId,
            });
        },
        _http_fetch_error_on_rust_side(fetchId: number) {
            sendMessageToMainThread({
                type: "http-fetch-error-on-rust-side",
                fetchId,
            });
        },
    };
}

export interface HttpFetchHandle {
    onHttpFetchErrorOnRustSide(fetchId: number): void;
    onNewHttpFetch({
        url,
        method,
        idBuffer,
    }: {
        url: string;
        method: string;
        idBuffer: SharedArrayBuffer;
    }): void;
    onHttpFetchSetHeader({
        fetchId,
        key,
        value,
    }: {
        fetchId: number;
        key: string;
        value: string;
    }): void;
    onHttpFetchStart({ fetchId }: { fetchId: number }): void;
    onHttpFetchPushRequestBodyChunk({
        fetchId,
        data,
    }: {
        fetchId: number;
        data: ArrayBuffer;
    }): void;
    onHttpFetchFinishRequestBodyStream({ fetchId }: { fetchId: number }): void;
}

export async function isSupportsRequestStreams() {
    const url = "data:a/a;charset=utf-8,";
    const supportsStreamsInRequestObjects = !new Request(url, {
        body: new ReadableStream(),
        // @ts-ignore
        duplex: "half",
        method: "POST",
    }).headers.has("Content-Type");

    if (!supportsStreamsInRequestObjects) return false;

    return fetch(url, {
        method: "POST",
        // @ts-ignore
        duplex: "half",
        body: new ReadableStream(),
    }).then(
        () => true,
        () => false,
    );
}

export function httpFetchHandleOnMainThread(
    supportsRequestStreams: boolean,
    newEventSystemHandle: NewEventSystemHandleOnMainThread,
    bufferPoolManager: BufferPoolHandleOnMainThread,
): HttpFetchHandle {
    function responseToEvent(
        fetchId: number,
        response: Response,
    ): NewSystemEvent {
        const headerEntries = Array.from(response.headers.entries());
        return {
            type: "http-fetch/on-response",
            fetchId: ["u32", fetchId],
            status: ["u16", response.status],
            headerCount: ["u16", headerEntries.length],
            headers: headerEntries.map(([key, value]) => {
                const keyBuffer = new TextEncoder().encode(key);
                const valueBuffer = new TextEncoder().encode(value);
                return {
                    keyByteLength: ["u16", keyBuffer.length],
                    key: ["bytes", new Uint8Array(keyBuffer).buffer],
                    valueByteLength: ["u16", valueBuffer.length],
                    value: ["bytes", new Uint8Array(valueBuffer).buffer],
                };
            }),
        };
    }
    function onResponse(fetchId: number, response: Response): void {
        newEventSystemHandle.sendEvent(responseToEvent(fetchId, response));
    }
    function onError(fetchId: number, error: unknown): void {
        const message = String(error);
        const messageBuffer = new TextEncoder().encode(message);
        newEventSystemHandle.sendEvent({
            type: "http-fetch/on-error",
            fetchId: ["u32", fetchId],
            messageByteLength: ["u32", messageBuffer.length],
            message: ["bytes", new Uint8Array(messageBuffer).buffer],
        });
    }

    if (supportsRequestStreams) {
        return new YesStreamHttpFetchHandle(
            onResponse,
            async function getResponseBodyBuffer(): Promise<{
                pooledBufferPtr: number;
                buffer: Uint8Array;
            }> {
                const buffer = await bufferPoolManager.getBuffer();
                return {
                    pooledBufferPtr: buffer.ptr,
                    buffer: buffer.view,
                };
            },
            function onResponseBodyChunk(
                fetchId: number,
                pooledBufferPtr: number,
                written: number,
            ): void {
                newEventSystemHandle.sendEvent({
                    type: "http-fetch/on-response-body-chunk",
                    fetchId: ["u32", fetchId],
                    pooledBufferPtr: ["u32", pooledBufferPtr],
                    written: ["u32", written],
                });
            },
            function onResponseBodyDone(fetchId: number): void {
                newEventSystemHandle.sendEvent({
                    type: "http-fetch/on-response-body-done",
                    fetchId: ["u32", fetchId],
                });
            },
            onError,
        );
    }
    return new NoStreamHttpFetchHandle(
        onResponse,
        async function onResponseBody(
            fetchId: number,
            body: Uint8Array,
        ): Promise<void> {
            let bodyWritten = 0;
            while (bodyWritten < body.length) {
                const buffer = await bufferPoolManager.getBuffer();
                buffer.view.set(body.slice(bodyWritten));
                const written = Math.min(
                    body.length - bodyWritten,
                    buffer.view.length,
                );

                newEventSystemHandle.sendEvent({
                    type: "http-fetch/on-response-body-chunk",
                    fetchId: ["u32", fetchId],
                    pooledBufferPtr: ["u32", buffer.ptr],
                    written: ["u32", written],
                });

                bodyWritten += written;
            }
            newEventSystemHandle.sendEvent({
                type: "http-fetch/on-response-body-done",
                fetchId: ["u32", fetchId],
            });
        },
        onError,
    );
}
