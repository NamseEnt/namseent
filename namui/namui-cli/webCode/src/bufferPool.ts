import { sendMessageToMainThread } from "./interWorkerProtocol";
import { NewEventSystemHandleOnMainThread } from "./newEventSystem";

export type PooledBuffer = {
    ptr: number;
    view: Uint8Array;
};

export function bufferPoolImports({ memory }: { memory: WebAssembly.Memory }) {
    return {
        _buffer_pool_new_buffer: (ptr: number, len: number) => {
            if (!(memory.buffer instanceof SharedArrayBuffer)) {
                throw new Error("memory.buffer must be SharedArrayBuffer");
            }

            sendMessageToMainThread({
                type: "buffer-pool-new-buffer",
                ptr,
                len,
            });
        },
    };
}

export class BufferPoolHandleOnMainThread {
    readonly bufferPool: PooledBuffer[] = [];
    readonly bufferWaiters: Array<(buffer: PooledBuffer) => void> = [];

    constructor(
        private readonly newEventSystemHandle: NewEventSystemHandleOnMainThread,
    ) {
        for (let i = 0; i < 128; i++) {
            this.requestBuffer();
        }
    }

    public pushBuffer(buffer: PooledBuffer) {
        const waiter = this.bufferWaiters.shift();
        if (waiter) {
            return waiter(buffer);
        }
        this.bufferPool.push(buffer);
    }

    public async getBuffer(): Promise<PooledBuffer> {
        this.requestBuffer();

        const buffer = this.bufferPool.pop();
        if (buffer) {
            return buffer;
        }

        return await new Promise<PooledBuffer>((resolve) => {
            this.bufferWaiters.push(resolve);
        });
    }

    private requestBuffer() {
        this.newEventSystemHandle.sendEvent({
            type: "buffer-pool/request-buffer",
        });
    }
}
