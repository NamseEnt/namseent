export type StrictArrayBuffer = ArrayBuffer & { buffer?: undefined };

export type RingBufferInput =
    | ["u8", number]
    | ["u16", number]
    | ["u32", number]
    | ["bytes", ArrayBuffer];

export type RingBufferInputs = RingBufferInput[];

export class RingBufferWriter {
    private writerIndex = 0;
    private isWriting = false;
    private queue: RingBufferInputs[] = [];
    public constructor(
        private readonly wasmMemory: ArrayBuffer,
        private readonly bufferPtr: number,
        private readonly bufferLen: number,
        private readonly writtenBuffer: SharedArrayBuffer,
    ) {}

    /**
     * It's FIFO.
     */
    public write(...inputs: RingBufferInputs) {
        this.queue.push(inputs);
        if (this.isWriting) {
            return;
        }
        this.runQueueLoop();
    }

    private async runQueueLoop() {
        this.isWriting = true;
        try {
            while (this.queue.length) {
                const inputs = this.queue.shift()!;
                await this.writeInputOneByOne(inputs);
            }
        } finally {
            this.isWriting = false;
        }
    }

    private async writeInputOneByOne(inputs: RingBufferInputs) {
        const totalByteLength = inputs.reduce((a, b) => {
            switch (b[0]) {
                case "u8":
                    return a + 1;
                case "u16":
                    return a + 2;
                case "u32":
                    return a + 4;
                case "bytes":
                    return a + b[1].byteLength;
            }
        }, 0);

        if (totalByteLength > this.bufferLen) {
            throw new Error(
                `The total byte length ${totalByteLength} is larger than the buffer length ${this.bufferLen}`,
            );
        }

        await this.waitForBufferAvailable(totalByteLength);

        for (const input of inputs) {
            this.writeInput(input);
        }

        Atomics.add(new Uint32Array(this.writtenBuffer), 0, totalByteLength);
        Atomics.notify(new Int32Array(this.writtenBuffer), 0);
    }

    private writeInput(
        input:
            | ["u8", number]
            | ["u16", number]
            | ["u32", number]
            | ["bytes", StrictArrayBuffer],
    ) {
        if (this.bufferLen <= this.writerIndex) {
            this.writerIndex = 0;
        }

        const type = input[0];
        let value: StrictArrayBuffer;
        switch (type) {
            case "u8": {
                value = new Uint8Array([input[1]]).buffer;
                break;
            }
            case "u16": {
                value = new Uint16Array([input[1]]).buffer;
                break;
            }
            case "u32": {
                value = new Uint32Array([input[1]]).buffer;
                break;
            }
            case "bytes": {
                value = input[1];
                break;
            }
            default: {
                throw new Error(`Unsupported type: ${type}`);
            }
        }

        const bufferRight = this.bufferLen - this.writerIndex;
        new Uint8Array(this.wasmMemory, this.bufferPtr).set(
            new Uint8Array(value, 0, Math.min(value.byteLength, bufferRight)),
            this.writerIndex,
        );

        if (value.byteLength <= bufferRight) {
            this.writerIndex += value.byteLength;
            return value.byteLength;
        }

        const left = value.byteLength - bufferRight;
        new Uint8Array(this.wasmMemory, this.bufferPtr).set(
            new Uint8Array(value, bufferRight),
            0,
        );

        this.writerIndex = left;
    }
    private async waitForBufferAvailable(byteLength: number) {
        while (true) {
            const written = Atomics.load(
                new Uint32Array(this.writtenBuffer),
                0,
            );
            const bufferAvailable = this.bufferLen - written;
            if (byteLength <= bufferAvailable) {
                return;
            }
            const { async, value } = Atomics.waitAsync(
                new Int32Array(this.writtenBuffer),
                0,
                written,
            );
            if (async) {
                await value;
            }
        }
    }
}
