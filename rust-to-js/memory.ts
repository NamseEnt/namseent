import { type Var } from "./impl";
import { Default } from "./default";

let nextPtr = 0;
const memories: { [ptr: number]: AllocatedMemory } = {};

export function alloc__alloc__exchange_malloc(size: Var, align: Var) {
    const ptr = nextPtr + (nextPtr % align.value);
    nextPtr = ptr + size.value;
    return new AllocatedMemory(ptr, size.value);
}

export class AllocatedMemory extends Default {
    bytes: Uint8Array;
    constructor(readonly ptr: number, readonly size: number) {
        super([new std__ptr__Unique(new std__ptr__NonNull(ptr))]);
        this.bytes = new Uint8Array(this.size);
        memories[ptr] = this;
    }
}

export class std__ptr__Unique extends Default {
    constructor(ptr: std__ptr__NonNull) {
        super([ptr]);
    }
}
export class std__ptr__NonNull extends Default {
    constructor(readonly ptr: number) {
        super([ptr]);
    }
    deref() {
        return new Deref(this.ptr);
    }
}

export class Deref {
    constructor(readonly ptr: number) {}
    assign(value: any) {
        const memory = memories[this.ptr];
        // memory.bytes.set(value);
    }
}

export class StackMemory {
    bytes: Uint8Array;
    nextPtr: number = 0;
    constructor(size: number) {
        this.bytes = new Uint8Array(size);
    }
    alloc(size: number): Uint8Array {
        const ptr = this.nextPtr;
        this.nextPtr += size;
        return this.bytes.subarray(ptr, ptr + size);
    }
}
