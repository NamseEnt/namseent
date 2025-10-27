const memory = new Uint8Array(1024 * 1024);
let stackLocalPtr = 0;

function ops_alloca(size) {
    const out = stackLocalPtr;
    stackLocalPtr += size;
    return out;
}

/**
 *
 * @param {Uint8Array} value
 * @param {number} ptr
 */
function ops_store(value, ptr) {
    memory.set(value, ptr);
}
