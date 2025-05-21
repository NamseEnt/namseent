import { HttpFetchHandle } from "./httpFetch";

export class NoStreamHttpFetchHandle implements HttpFetchHandle {
    private nextId = 1;
    private readonly requests = new Map<number, Request>();
    private readonly requestBodies = new Map<
        number,
        {
            written: number;
            buffer: Uint8Array;
        }
    >();
    private readonly abortControllers = new Map<number, AbortController>();

    constructor(
        private readonly onResponse: (
            fetchId: number,
            response: Response,
        ) => void,
        private readonly onResponseBody: (
            fetchId: number,
            body: Uint8Array,
        ) => void,
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
        const request = new Request(url, { method });

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
        const contentLength = request.headers.get("content-length");
        if (!contentLength) {
            return;
        }
        const buffer = new Uint8Array(Number(contentLength));
        this.requestBodies.set(fetchId, {
            written: 0,
            buffer,
        });
    }

    onHttpFetchPushRequestBodyChunk({
        fetchId,
        data,
    }: {
        fetchId: number;
        data: ArrayBuffer;
    }): void {
        const requestBody = this.requestBodies.get(fetchId);
        if (!requestBody) {
            throw new Error(`Request body not found: ${fetchId}`);
        }
        requestBody.buffer.set(new Uint8Array(data), requestBody.written);
        requestBody.written += data.byteLength;
    }

    async onHttpFetchFinishRequestBodyStream({ fetchId }: { fetchId: number }) {
        const requestBody = this.requestBodies.get(fetchId);

        const abortController = new AbortController();
        this.abortControllers.set(fetchId, abortController);

        try {
            const response = await fetch(this.requests.get(fetchId)!, {
                body: requestBody?.buffer,
                signal: abortController.signal,
            });

            this.onResponse(fetchId, response);
            const responseBody = await response.arrayBuffer();
            this.onResponseBody(fetchId, new Uint8Array(responseBody));
        } catch (error) {
            console.error(error);
            this.onError(fetchId, error);
        } finally {
            this.cleanUpFetch(fetchId);
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

    onHttpFetchErrorOnRustSide(fetchId: number): void {
        this.cleanUpFetch(fetchId);
    }
}
