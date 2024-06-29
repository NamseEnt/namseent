export function storageImports({
    memory,
    storageProtocolBuffer,
}: {
    memory: WebAssembly.Memory;
    storageProtocolBuffer: SharedArrayBuffer;
}) {
    const i32 = new Int32Array(storageProtocolBuffer);

    function request(...args: number[]) {
        for (let i = 0; i < args.length; i++) {
            i32[i] = args[i];
        }
        const requestType = i32[0];
        Atomics.notify(i32, 0);
        Atomics.wait(i32, 0, requestType);
    }

    return {
        /// # Returns
        /// 0: not found
        /// non-zero: file descriptor
        _storage_open_read(key_ptr: number, key_len: number): number {
            request(1, key_ptr, key_len);
            return i32[1];
        },
        /// # Parameters
        /// - `is_done`:
        ///     - 0: not done
        ///     - non-zero: done
        _storage_read(
            fd: number,
            buffer_ptr: number,
            buffer_len: number,
            read_byte_length_ptr: number,
            is_done_ptr: number,
        ) {
            request(2, fd, buffer_ptr, buffer_len);
            new Int32Array(memory.buffer, read_byte_length_ptr)[0] = i32[1];
            new Int32Array(memory.buffer, is_done_ptr)[0] = i32[2];
        },
        _storage_open_write(key: number, keyLen: number): number {
            request(3, key, keyLen);
            return i32[1];
        },
        _storage_write(
            fd: number,
            buffer_ptr: number,
            buffer_len: number,
        ): number {
            request(4, fd, buffer_ptr, buffer_len);
            return i32[1];
        },
        _storage_flush(fd: number) {
            request(5, fd);
        },
        _storage_close(fd: number) {
            request(6, fd);
        },
        _storage_delete(key: number, keyLen: number) {
            request(7, key, keyLen);
        },
    };
}
