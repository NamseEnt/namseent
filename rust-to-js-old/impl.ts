let stackPtr = 0;
const stackCheckpoints: number[] = [];
let memory = new Uint8Array(1024 * 1024);

class Fat {
    constructor(public readonly ptr: number, public readonly size: number) {}
}

export function stackPush() {
    stackCheckpoints.push(stackPtr);
}

export function stackPop() {
    stackPtr = stackCheckpoints.pop()!;
}

export function stackAlloc(size: number) {
    const ptr = stackPtr;
    stackPtr += size;
    return new Fat(ptr, size);
}

export function assign(dest: Fat, src: Fat) {
    memory.copyWithin(dest.ptr, src.ptr, src.ptr + src.size);
}

export function stackAllocArray(items: Fat[]) {
    const ptr = stackPtr;
    const totalSize = items.length ? items.length * items[0].size : 0;
    stackPtr += totalSize;
    return new Fat(ptr, totalSize);
}

export function stackAllocBytes(bytes: Uint8Array) {
    const ptr = stackPtr;
    memory.set(bytes, stackPtr);
    stackPtr += bytes.length;
    return new Fat(ptr, bytes.length);
}

export function stackAllocInt8(value: number) {
    const ptr = stackPtr;
    memory.set(new Int8Array([value]), stackPtr);
    stackPtr += 1;
    return new Fat(ptr, 1);
}

export function stackAllocInt16(value: number) {
    const ptr = stackPtr;
    memory.set(new Int16Array([value]), stackPtr);
    stackPtr += 2;
    return new Fat(ptr, 2);
}

export function stackAllocInt32(value: number) {
    const ptr = stackPtr;
    memory.set(new Int32Array([value]), stackPtr);
    stackPtr += 4;
    return new Fat(ptr, 4);
}

export function stackAllocUint8(value: number) {
    const ptr = stackPtr;
    memory.set(new Uint8Array([value]), stackPtr);
    stackPtr += 1;
    return new Fat(ptr, 1);
}

export function stackAllocUint16(value: number) {
    const ptr = stackPtr;
    memory.set(new Uint16Array([value]), stackPtr);
    stackPtr += 2;
    return new Fat(ptr, 2);
}

export function stackAllocUint32(value: number) {
    const ptr = stackPtr;
    memory.set(new Uint32Array([value]), stackPtr);
    stackPtr += 4;
    return new Fat(ptr, 4);
}

export function std__io___print() {
    // dummy
    return stackAlloc(0);
}
