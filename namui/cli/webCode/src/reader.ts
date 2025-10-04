// Bincode reader utility functions
export class BincodeReader {
    private view: DataView;
    private offset: number = 0;

    constructor(buffer: ArrayBuffer, byteOffset?: number, byteLength?: number) {
        this.view = new DataView(buffer, byteOffset, byteLength);
    }

    getOffset(): number {
        return this.offset;
    }

    readU8(): number {
        const value = this.view.getUint8(this.offset);
        this.offset += 1;
        return value;
    }

    readU32(): number {
        const value = this.view.getUint32(this.offset, true);
        this.offset += 4;
        return value;
    }

    readU64(): bigint {
        const value = this.view.getBigUint64(this.offset, true);
        this.offset += 8;
        return value;
    }

    readI32(): number {
        const value = this.view.getInt32(this.offset, true);
        this.offset += 4;
        return value;
    }

    // Zigzag varint i32 for bincode standard()
    readVarintI32(): number {
        const zigzag = this.readVarintU32();
        // Zigzag decode: (n >> 1) ^ -(n & 1)
        return (zigzag >>> 1) ^ -(zigzag & 1);
    }

    readF32(): number {
        const value = this.view.getFloat32(this.offset, true);
        this.offset += 4;
        return value;
    }

    readF64(): number {
        const value = this.view.getFloat64(this.offset, true);
        this.offset += 8;
        return value;
    }

    readString(): string {
        const len = Number(this.readVarintU64());
        const bytes = new Uint8Array(this.view.buffer, this.view.byteOffset + this.offset, len);
        const copiedBuffer = new ArrayBuffer(len);
        new Uint8Array(copiedBuffer).set(bytes);
        this.offset += len;
        return new TextDecoder().decode(new Uint8Array(copiedBuffer));
    }

    readBool(): boolean {
        return this.readU8() !== 0;
    }

    readU16(): number {
        const value = this.view.getUint16(this.offset, true);
        this.offset += 2;
        return value;
    }

    // Varint encoding for unsigned integers
    readVarintU32(): number {
        const first = this.readU8();
        if (first < 251) {
            return first;
        } else if (first === 251) {
            return this.readU16();
        } else if (first === 252) {
            return this.readU32();
        } else {
            throw new Error(`Unexpected varint marker for u32: ${first}`);
        }
    }

    readVarintU64(): number {
        const first = this.readU8();
        if (first < 251) {
            return first;
        } else if (first === 251) {
            return this.readU16();
        } else if (first === 252) {
            return this.readU32();
        } else if (first === 253) {
            // u64 - convert to number (may lose precision for very large values)
            return Number(this.readU64());
        } else {
            throw new Error(`Unexpected varint marker for u64: ${first}`);
        }
    }
}
