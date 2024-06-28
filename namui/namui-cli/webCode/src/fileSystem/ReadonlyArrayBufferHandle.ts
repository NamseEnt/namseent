export class ReadonlyArrayBufferHandle {
    public readonly byteLength: number;
    private cursor = 0;
    constructor(private readonly buffer: ArrayBuffer) {
        this.byteLength = buffer.byteLength;
    }
    /**
     * no cursor update
     */
    public pread(dest: Uint8Array, offset: number): number {
        new Uint8Array(this.buffer, offset).set(dest);
        return Math.min(this.buffer.byteLength - offset, dest.byteLength);
    }
    /**
     * use cursor
     */
    public read(dest: Uint8Array): number {
        const readLength = this.pread(dest, this.cursor);
        this.cursor += readLength;
        return readLength;
    }

    public getCursor(): number {
        return this.cursor;
    }

    public seek(position: number) {
        this.cursor = position;
    }
}
