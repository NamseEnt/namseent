import { HttpFetchHandle } from "./httpFetch";

export class YesStreamHttpFetchHandle implements HttpFetchHandle {
    private nextId = 1;
    private readonly requests = new Map<number, Request>();
    private readonly requestBodies = new Map<
        number,
        {
            stream: WritableStream;
            isSomeoneWriting: boolean;
            queuedChunks: ArrayBuffer[];
            isOver: boolean;
        }
    >();
    private readonly abortControllers = new Map<number, AbortController>();

    constructor(
        private readonly onResponse: (
            fetchId: number,
            response: Response,
        ) => void,
        private readonly getResponseBodyBuffer: () => Promise<{
            pooledBufferPtr: number;
            buffer: Uint8Array;
        }>,
        private readonly onResponseBodyChunk: (
            fetchId: number,
            pooledBufferPtr: number,
            written: number,
        ) => void,
        private readonly onResponseBodyDone: (fetchId: number) => void,
        private readonly onError: (fetchId: number, error: unknown) => void,
    ) {}

    onNewHttpFetch({
        url,
        method,
        idBuffer,
    }: {
        url: string;
        method: string;
        idBuffer: SharedArrayBuffer;
    }): void {
        const request = new Request(url, {
            method,
        });

        const id = this.nextId++;
        this.requests.set(id, request);

        new Uint32Array(idBuffer)[0] = id;
        Atomics.notify(new Int32Array(idBuffer), 0);
    }

    onHttpFetchSetHeader({
        fetchId,
        key,
        value,
    }: {
        fetchId: number;
        key: string;
        value: string;
    }): void {
        const request = this.requests.get(fetchId);
        if (!request) {
            throw new Error(`Request not found: ${fetchId}`);
        }
        request.headers.set(key, value);
    }

    onHttpFetchStart({ fetchId }: { fetchId: number }): void {
        const request = this.requests.get(fetchId);
        if (!request) {
            throw new Error(`Request not found: ${fetchId}`);
        }
        const { readable, writable } = new TransformStream();

        this.requestBodies.set(fetchId, {
            isSomeoneWriting: false,
            stream: writable,
            queuedChunks: [],
            isOver: false,
        });

        const abortController = new AbortController();
        this.abortControllers.set(fetchId, abortController);

        (async () => {
            try {
                const requestBody = ["GET", "HEAD", "OPTIONS"].includes(
                    request.method,
                )
                    ? undefined
                    : request.url.startsWith("http://")
                    ? await fullRead(readable)
                    : readable;

                const response = await fetch(request, {
                    body: requestBody,
                    signal: abortController.signal,
                    // @ts-ignore
                    duplex: "half",
                });
                this.onResponse(fetchId, response);

                const { body } = response;
                if (!body) {
                    this.onResponseBodyDone(fetchId);
                    return;
                }

                const reader = body.getReader({
                    mode: "byob",
                });

                while (!this.isCleanedUp(fetchId)) {
                    const { buffer, pooledBufferPtr } =
                        await this.getResponseBodyBuffer();

                    // NOTE: Currently byob doesn't support to write to shared buffer directly
                    const tempBuffer = new Uint8Array(buffer.length);

                    const { value, done } = await reader.read(tempBuffer);

                    if (!value) {
                        throw new Error("Response Stream Canceled");
                    }

                    buffer.set(value);

                    this.onResponseBodyChunk(
                        fetchId,
                        pooledBufferPtr,
                        value.byteLength,
                    );

                    if (done) {
                        this.onResponseBodyDone(fetchId);
                        break;
                    }
                }
            } catch (error) {
                console.error(request.url, error);
                this.onError(fetchId, error);
            } finally {
                this.cleanUpFetch(fetchId);
            }
        })();
    }

    async onHttpFetchPushRequestBodyChunk({
        fetchId,
        data,
    }: {
        fetchId: number;
        data: ArrayBuffer;
    }): Promise<void> {
        const requestBody = this.requestBodies.get(fetchId);
        if (!requestBody) {
            throw new Error(`Request body not found: ${fetchId}`);
        }

        requestBody.queuedChunks.push(data);

        if (requestBody.isSomeoneWriting) {
            return;
        }
        requestBody.isSomeoneWriting = true;
        try {
            const writer = requestBody.stream.getWriter();

            while (requestBody.queuedChunks.length) {
                const chunk = requestBody.queuedChunks.shift()!;
                await writer.write(chunk);
            }

            if (requestBody.isOver) {
                writer.close();
            } else {
                writer.releaseLock();
            }

            requestBody.isSomeoneWriting = false;
        } catch (error) {
            console.error(error);
            this.onError(fetchId, error);
            this.cleanUpFetch(fetchId);
        }
    }

    async onHttpFetchFinishRequestBodyStream({ fetchId }: { fetchId: number }) {
        const requestBody = this.requestBodies.get(fetchId);
        if (!requestBody) {
            throw new Error(`Request body not found: ${fetchId}`);
        }

        requestBody.isOver = true;

        if (!requestBody.isSomeoneWriting) {
            try {
                await requestBody.stream.getWriter().close();
            } catch (error) {
                console.error(error);
                this.onError(fetchId, error);
                this.cleanUpFetch(fetchId);
            }
        }
    }

    cleanUpFetch(fetchId: number) {
        this.requests.delete(fetchId);
        this.requestBodies.delete(fetchId);
        const abortController = this.abortControllers.get(fetchId);
        if (abortController) {
            abortController.abort();
            this.abortControllers.delete(fetchId);
        }
    }

    isCleanedUp(fetchId: number) {
        return !this.requests.has(fetchId);
    }

    onHttpFetchErrorOnRustSide(fetchId: number): void {
        this.cleanUpFetch(fetchId);
    }
}
async function fullRead(
    readable: ReadableStream<ArrayBuffer>,
): Promise<Uint8Array> {
    const reader = readable.getReader();
    const chunks: Uint8Array[] = [];
    let totalLength = 0;
    while (true) {
        const { value, done } = await reader.read();
        if (done) {
            break;
        }
        chunks.push(new Uint8Array(value));
        totalLength += value.byteLength;
    }
    const result = new Uint8Array(totalLength);
    let offset = 0;
    for (const chunk of chunks) {
        result.set(chunk, offset);
        offset += chunk.length;
    }
    return result;
}
