export type StrictArrayBuffer = ArrayBuffer & { buffer?: undefined };

export class RingBufferWriter {
    private writerIndex = 0;
    public constructor(
        private readonly wasmMemory: ArrayBuffer,
        private readonly bufferPtr: number,
        private readonly bufferLen: number,
        private readonly writtenBuffer: SharedArrayBuffer,
    ) {}

    public async write(
        ...tuples: (
            | ["u8", number]
            | ["u16", number]
            | ["u32", number]
            | ["bytes", ArrayBuffer]
        )[]
    ) {
        const totalByteLength = tuples.reduce((a, b) => {
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

        for (const tuple of tuples) {
            this.writeTuple(tuple);
        }

        Atomics.add(new Uint32Array(this.writtenBuffer), 0, totalByteLength);
        Atomics.notify(new Int32Array(this.writtenBuffer), 0);
    }

    writeTuple(
        tuple:
            | ["u8", number]
            | ["u16", number]
            | ["u32", number]
            | ["bytes", StrictArrayBuffer],
    ) {
        if (this.bufferLen <= this.writerIndex) {
            this.writerIndex = 0;
        }

        const type = tuple[0];
        let value: StrictArrayBuffer;
        switch (type) {
            case "u8": {
                value = new Uint8Array([tuple[1]]).buffer;
                break;
            }
            case "u16": {
                value = new Uint16Array([tuple[1]]).buffer;
                break;
            }
            case "u32": {
                value = new Uint32Array([tuple[1]]).buffer;
                break;
            }
            case "bytes": {
                value = tuple[1];
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
    async waitForBufferAvailable(byteLength: number) {
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
