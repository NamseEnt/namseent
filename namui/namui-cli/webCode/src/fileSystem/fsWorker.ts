import { wasi } from "@bjorn3/browser_wasi_shim";
import { fsFuncs } from ".";
import { WorkerMessagePayload } from "../interWorkerProtocol";
import bundleSqliteUrl from "bundle.sqlite?url";
import { ReadonlyArrayBufferHandle } from "./ReadonlyArrayBufferHandle";

const waitDisconnectMap = new Map<number, () => void>();
let wasmMemoryResolve: (value: WebAssembly.Memory) => void;
const wasmMemory: Promise<WebAssembly.Memory> = new Promise((resolve) => {
    wasmMemoryResolve = resolve;
});
const bundleSqlite = fetch(bundleSqliteUrl).then((response) =>
    response.arrayBuffer(),
);

self.onmessage = async (message) => {
    const payload = message.data as WorkerMessagePayload;
    console.log("payload", payload);

    switch (payload.type) {
        case "fs-init":
            {
                console.log("fs-init");
                wasmMemoryResolve(payload.wasmMemory);
            }
            break;
        case "fs-thread-disconnect":
            {
                const { threadId: _ } = payload;
                // TODO
            }
            break;
        case "fs-thread-connect":
            {
                const { protocolBuffer, threadId } = payload;
                const int32 = new Int32Array(protocolBuffer);

                const waitDisconnect = new Promise<typeof DISCONNECTED>(
                    (resolve) => {
                        waitDisconnectMap.set(threadId, () =>
                            resolve(DISCONNECTED),
                        );
                    },
                );

                const implement = implementation({
                    wasmMemory: await wasmMemory,
                    bundleSqlite: await bundleSqlite,
                    threadId,
                });

                while (true) {
                    const result = await Promise.any([
                        waitForRequest(int32),
                        waitDisconnect,
                    ]);
                    if (result == DISCONNECTED) {
                        break;
                    }
                    const argsCount = int32[0];

                    const args = Array.from(int32.slice(1, argsCount + 1));
                    const noLog = !(
                        fsFuncs[args[0]] === "fd_write" &&
                        [1, 2].includes(args[1])
                    );
                    noLog &&
                        console.log(
                            `fs::${threadId} function name`,
                            fsFuncs[args[0]],
                            args,
                        );
                    const response = await implement[args[0]](...args.slice(1));
                    noLog && console.log(`fs::${threadId} response`, response);
                    int32[1] = response || 0;
                    int32[0] = 0;
                    Atomics.notify(int32, 0);
                }
            }
            break;
    }
};

const REQUEST_ARRIVED = "REQUEST_ARRIVED";
const DISCONNECTED = "DISCONNECTED";

async function waitForRequest(
    int32: Int32Array,
): Promise<typeof REQUEST_ARRIVED> {
    await Atomics.waitAsync(int32, 0, 0).value;
    return REQUEST_ARRIVED;
}

function implementation({
    wasmMemory,
    bundleSqlite,
    threadId,
}: {
    wasmMemory: WebAssembly.Memory;
    bundleSqlite: ArrayBuffer;
    threadId: number;
}): Record<
    number,
    (...args: number[]) => number | void | Promise<number> | Promise<void>
> {
    const bundle = new ReadonlyArrayBufferHandle(bundleSqlite);
    const fds: (Fd | undefined)[] = [
        new OpenFile(new File([])),
        ConsoleStdout.lineBuffered((msg) =>
            console.log(`[WASI stdout(tid: ${threadId})] ${msg}`),
        ),
        ConsoleStdout.lineBuffered((msg) =>
            console.log(`[WASI stderr(tid: ${threadId})] ${msg}`),
        ),
        new PreopenDirectory(
            "",
            new Map([
                [
                    "bundle.sqlite",
                    new File(bundleSqlite, {
                        readonly: true,
                    }),
                ],
            ]),
        ),
    ];

    const dataView = () => new DataView(wasmMemory.buffer);
    const uint8 = () => new Uint8Array(wasmMemory.buffer);

    const implement = {
        /**
         * The fd_advise() function is used to provide advice to the operating system about the intended usage of a file.
         * This advice can help the system optimize performance or memory usage based on the specified parameters.
         */
        fd_advise(
            fd: number,
            _offset: number, // bigint,
            _len: number, // bigint,
            _advice: number,
        ): number {
            return fds[fd] ? wasi.ERRNO_SUCCESS : wasi.ERRNO_BADF;
        },
        /**
         * The fd_allocate function is used to allocate additional space for a file descriptor.
         * It allows extending the size of a file or buffer associated with the file descriptor.
         */
        fd_allocate(
            _fd: number,
            _offset: number, // bigint,
            _len: number, // bigint
        ): number {
            throw new Error("Not implemented");
            // const handle = fdAsFileHandle(fd);
            // if (!handle) {
            //     return wasi.ERRNO_BADF;
            // }
            // handle.truncate(offset + len);
            // return wasi.ERRNO_SUCCESS;
        },
        /**
         * The fd_close() function is used to close an open file descriptor.
         * For sockets, this function will flush the data before closing the socket.
         */
        fd_close(fd: number): number {
            const handle = fds[fd];
            if (!handle) {
                return wasi.ERRNO_BADF;
            }
            handle.fd_close();
            fds[fd] = undefined;
            return wasi.ERRNO_SUCCESS;
        },
        /**
         * The fd_datasync() function is used to synchronize the file data associated with a file descriptor to disk.
         * It ensures that any modified data for the file is written to the underlying storage device.
         */
        fd_datasync(_fd: number): number {
            throw new Error("Not implemented");
            // const handle = fdAsFileHandle(fd);
            // if (!handle) {
            //     return wasi.ERRNO_BADF;
            // }
            // handle.flush();
            // return wasi.ERRNO_SUCCESS;
        },
        /**
         * The fd_fdstat_get() function is used to retrieve the metadata of a file descriptor.
         * It provides information about the state of the file descriptor, such as its rights, flags, and file type.
         *
         * In POSIX systems, file descriptors are small, non-negative integers used to represent open files,
         * sockets, or other I/O resources.
         * They serve as handles that allow processes to read from or write to these resources.
         * The fd_fdstat_get() function allows applications to retrieve information about a specific file descriptor,
         * gaining insights into its properties and characteristics.
         */
        fd_fdstat_get(fd: number, fdstat_ptr: number): number {
            const fdstat = new wasi.Fdstat(
                [1, 2].includes(fd)
                    ? wasi.FILETYPE_CHARACTER_DEVICE
                    : wasi.FILETYPE_REGULAR_FILE,
                0,
            );
            fdstat.write_bytes(dataView(), fdstat_ptr);
            return wasi.ERRNO_SUCCESS;
        },
        /**
         * The fd_fdstat_set_flags() function is used to set the file descriptor flags for a given file descriptor.
         * File descriptor flags modify the behavior and characteristics of the file descriptor,
         * allowing applications to customize its behavior according to specific requirements.
         *
         * In POSIX systems, file descriptors are associated with a set of flags that control various aspects of their behavior.
         * These flags provide additional control over file descriptor operations,
         * such as non-blocking mode, close-on-exec behavior, or file status flags.
         * The fd_fdstat_set_flags() function allows applications to modify these flags for a particular file descriptor,
         * altering its behavior as needed.
         */
        fd_fdstat_set_flags(_fd: number, _flags: number): number {
            return wasi.ERRNO_NOTSUP;
        },
        /**
         * The fd_fdstat_set_rights() function is used to set the rights of a file descriptor.
         * It allows modifying the access rights associated with the file descriptor
         * by applying new rights or removing existing rights.
         *
         * In POSIX systems, file descriptors are associated with access rights that define the operations
         * that can be performed on the file or resource represented by the descriptor.
         * These access rights control actions such as reading, writing, or seeking within the file.
         * The fd_fdstat_set_rights() function enables applications to modify the access rights
         * associated with a file descriptor, restricting or expanding the available operations.
         */
        fd_fdstat_set_rights(
            _fd: number,
            _fs_rights_base: number, // bigint,
            _fs_rights_inheriting: number, // bigint,
        ): number {
            return wasi.ERRNO_NOTSUP;
        },
        /**
         * The fd_filestat_get() function is used to retrieve the metadata of an open file identified by a file descriptor.
         * It provides information about the file's attributes, such as its size, timestamps, and permissions.
         *
         * In POSIX systems, file descriptors are used to perform I/O operations on files.
         * The fd_filestat_get() function allows applications to obtain the metadata of an open file,
         * providing access to important details about the file.
         */
        fd_filestat_get(fd: number, filestat_ptr: number): number {
            const handle = fds[fd];
            if (!handle) {
                return wasi.ERRNO_BADF;
            }
            const { ret, filestat } = handle.fd_filestat_get();
            if (filestat != null) {
                filestat.write_bytes(dataView(), filestat_ptr);
            }
            return ret;
        },

        /**
         * The fd_filestat_set_size() function is used to modify the size of an open file identified by a file descriptor.
         * It allows adjusting the size of the file and zeroing out any newly allocated bytes.
         *
         * In POSIX systems, files have a size attribute that represents the amount of data stored in the file.
         * The fd_filestat_set_size() function enables applications to change the size of an open file.
         * When increasing the file size, any newly allocated bytes are automatically filled with zeros.
         */
        fd_filestat_set_size(
            _fd: number,
            _size: number, // bigint
        ): number {
            throw new Error("Not implemented");
        },
        /**
         * In POSIX systems, files have associated timestamp metadata that stores information
         * about the file's access and modification times.
         * The fd_filestat_set_times() function enables applications to update these timestamps.
         * It allows setting the last accessed and last modified times to specific values or using the current time.
         */
        fd_filestat_set_times(
            _fd: number,
            _atim: number, // bigint,
            _mtim: number, // bigint,
            _fst_flags: number,
        ): number {
            return wasi.ERRNO_NOTSUP;
        },
        /**
         * The fd_pread() function is used to read data from a file identified
         * by the provided file descriptor (fd) at a specified offset (offset).
         * Unlike regular reading operations, fd_pread() does not update the file cursor, making it a stateless operation.
         * The function reads data into the provided buffers (iovs) and returns the number of bytes read.
         *
         * In POSIX systems, file reading operations typically involve updating the file cursor,
         * which determines the next position from which data will be read.
         * However, fd_pread() allows reading data from a specific offset without modifying the file cursor's position.
         * This can be useful in scenarios where applications need to read data from a file
         * at a specific location without altering the cursor's state.
         */
        fd_pread(
            fd: number,
            iovs_ptr: number,
            iovs_len: number,
            offset: number, // bigint,
            nread_ptr: number,
        ): number {
            throw new Error("Not implemented");
            // if (![3].includes(fd)) {
            //     return wasi.ERRNO_BADF;
            // }

            // const iovecs = wasi.Iovec.read_bytes_array(
            //     dataView(),
            //     iovs_ptr,
            //     iovs_len,
            // );

            // let totalRead = 0;
            // for (const iovec of iovecs) {
            //     const read = bundle.pread(
            //         new Uint8Array(wasmMemory, iovec.buf, iovec.buf_len),
            //         offset,
            //     );
            //     totalRead += read;
            //     offset += read;
            //     if (read !== iovec.buf_len) {
            //         break;
            //     }
            // }
            // dataView().setUint32(nread_ptr, totalRead, true);
            // return wasi.ERRNO_SUCCESS;
        },
        /**
         * The fd_prestat_get() function is used to retrieve metadata about a preopened file descriptor (fd).
         * Preopened file descriptors represent files or directories that are provided to a WebAssembly module at startup.
         * This function allows obtaining information about such preopened file descriptors.
         *
         * The function takes the file descriptor as input and writes the corresponding metadata
         * into the provided buffer (buf) of type __wasi_prestat.
         * The metadata includes information such as the type of the preopened resource.
         */
        fd_prestat_get(fd: number, buf_ptr: number): number {
            const handle = fds[fd];
            if (!handle) {
                return wasi.ERRNO_BADF;
            }
            const { ret, prestat } = handle.fd_prestat_get();
            if (prestat) {
                prestat.write_bytes(dataView(), buf_ptr);
            }
            return ret;
        },
        /**
         * The fd_prestat_dir_name() function is used to retrieve the directory name
         * associated with a preopened file descriptor (fd).
         * It retrieves the directory name from the file system and writes it
         * into the provided buffer (path) up to the specified length (path_len).
         * The function returns an error code indicating the success or failure of the operation.
         *
         * When working with preopened file descriptors,
         * which represent files opened by the WebAssembly module's host environment,
         * it can be useful to obtain information about the directory from which the file was opened.
         * The fd_prestat_dir_name() function provides a convenient way to retrieve the directory name
         * associated with a preopened file descriptor.
         */
        fd_prestat_dir_name(
            fd: number,
            path_ptr: number,
            path_len: number,
        ): number {
            const handle = fds[fd];
            if (!handle) {
                return wasi.ERRNO_BADF;
            }
            const { ret, prestat } = handle.fd_prestat_get();
            if (prestat == null) {
                return ret;
            }
            const prestat_dir_name = prestat.inner.pr_name;

            uint8().set(prestat_dir_name.slice(0, path_len), path_ptr);

            return prestat_dir_name.byteLength > path_len
                ? wasi.ERRNO_NAMETOOLONG
                : wasi.ERRNO_SUCCESS;
        },
        /**
         * The fd_pwrite() function is used to write data to a file identified by the provided file descriptor (fd)
         * without modifying its offset.
         * It takes an array of __wasi_ciovec_t structures (iovs) describing the buffers from which data will be read,
         * the number of vectors (iovs_len) in the iovs array,
         * the offset indicating the position at which the data will be written,
         * and a pointer to store the number of bytes written.
         * The function writes the data to the file at the specified offset and returns the number of bytes written.
         *
         * In POSIX systems, writing data to a file typically involves updating the file cursor,
         * which determines the next position at which data will be written.
         * However, the fd_pwrite() function provides a way to write data to a file without adjusting its offset.
         * This can be useful in scenarios where applications need to write data at a specific location
         * in a file without modifying the file cursor's state.
         */
        fd_pwrite(
            fd: number,
            iovs_ptr: number,
            iovs_len: number,
            offset: number, // bigint,
            nwritten_ptr: number,
        ): number {
            throw new Error("Not implemented");
            // const handle = fdAsFileHandle(fd);
            // if (!handle) {
            //     return wasi.ERRNO_BADF;
            // }
            // const iovecs = wasi.Ciovec.read_bytes_array(
            //     dataView(),
            //     iovs_ptr,
            //     iovs_len,
            // );
            // let totalWritten = 0;
            // for (const iovec of iovecs) {
            //     const written = handle.write(
            //         new Uint8Array(wasmMemory, iovec.buf, iovec.buf_len),
            //         {
            //             at: offset,
            //         },
            //     );
            //     totalWritten += written;
            //     offset += written;
            //     if (written != iovec.buf_len) {
            //         break;
            //     }
            // }
            // dataView().setUint32(nwritten_ptr, totalWritten, true);
            // return wasi.ERRNO_SUCCESS;
        },
        /**
         * The fd_read() function is used to read data from a file identified by the provided file descriptor (fd).
         * It takes an array of __wasi_iovec_t structures (iovs) describing the buffers where the data will be stored,
         * the number of vectors (iovs_len) in the iovs array, and a pointer to store the number of bytes read.
         * The function reads data from the file into the specified buffers and returns the number of bytes read.
         *
         * In POSIX systems, reading data from a file typically involves updating the file cursor,
         * which determines the next position from which data will be read.
         * The fd_read() function allows reading data from a file without modifying the file cursor's position.
         * This can be useful in scenarios where applications need to read data from a specific location
         * in a file without altering the cursor's state.
         */
        fd_read(
            fd: number,
            iovs_ptr: number,
            iovs_len: number,
            nread_ptr: number,
        ): number {
            const handle = fds[fd];
            if (!handle) {
                return wasi.ERRNO_BADF;
            }
            const iovecs = wasi.Iovec.read_bytes_array(
                dataView(),
                iovs_ptr,
                iovs_len,
            );
            let totalRead = 0;
            for (const iovec of iovecs) {
                const result = handle.fd_read(
                    new Uint8Array(wasmMemory.buffer, iovec.buf, iovec.buf_len),
                );
                if (result.ret !== wasi.ERRNO_SUCCESS) {
                    return result.ret;
                }
                totalRead += result.readCount;
                if (result.readCount !== iovec.buf_len) {
                    break;
                }
            }
            dataView().setUint32(nread_ptr, totalRead, true);
            return wasi.ERRNO_SUCCESS;
        },
        /**
         * The fd_readdir() function reads directory entries from a directory identified by the provided file descriptor (fd).
         * It stores the directory entries in the buffer specified by buf and returns the number of bytes stored in the buffer via the bufused pointer.
         */
        async fd_readdir(
            fd: number,
            buf: number,
            buf_len: number,
            cookie: number, // bigint,
            bufused_ptr: number,
        ): Promise<number> {
            return wasi.ERRNO_BADF;
        },
        /**
         * The fd_renumber() function atomically copies a file descriptor from one location to another.
         * It ensures that the copying operation is performed atomically and returns an Errno value indicating the success or failure of the operation.
         */
        fd_renumber(fd: number, to: number) {
            return wasi.ERRNO_NOTSUP;
        },
        /**
         * The fd_seek() function updates the offset of a file descriptor.
         * It allows you to adjust the offset by a specified number of bytes relative to a given position.
         */
        fd_seek(
            fd: number,
            offset: number, // bigint,
            whence: number,
            offset_out_ptr: number,
        ): number {
            const handle = fds[fd];
            if (!handle) {
                return wasi.ERRNO_BADF;
            }
            const { ret, offset: offset_out } = handle.fd_seek(
                BigInt(offset),
                whence,
            );
            if (ret !== wasi.ERRNO_SUCCESS) {
                return ret;
            }
            dataView().setBigUint64(offset_out_ptr, offset_out, true);
            return wasi.ERRNO_SUCCESS;
        },
        /**
         * The fd_sync() function synchronizes the file and metadata associated with a file descriptor to disk.
         * This ensures that any changes made to the file and its metadata are persisted and visible to other processes.
         */
        fd_sync(fd: number): number {
            throw new Error("Not implemented");
            // const handle = fdAsFileHandle(fd);
            // if (!handle) {
            //     return wasi.ERRNO_BADF;
            // }
            // handle.flush();
            // return wasi.ERRNO_SUCCESS;
        },
        /**
         * The fd_tell() function retrieves the current offset of the specified file descriptor
         * relative to the start of the file.
         */
        fd_tell(fd: number, offset_ptr: number): number {
            const handle = fds[fd];
            if (!handle) {
                return wasi.ERRNO_BADF;
            }
            const { ret, offset } = handle.fd_tell();
            if (ret !== wasi.ERRNO_SUCCESS) {
                return ret;
            }
            dataView().setBigUint64(offset_ptr, offset, true);
            return wasi.ERRNO_SUCCESS;
        },
        /**
         * The fd_write() function writes data from one or more buffers to the specified file descriptor.
         * It is similar to the POSIX write() function, but with additional support for
         * writing multiple non-contiguous buffers in a single function call using the iovs parameter.
         */
        fd_write(
            fd: number,
            iovs_ptr: number,
            iovs_len: number,
            nwritten_ptr: number,
        ): number {
            const handle = fds[fd];
            if (!handle) {
                return wasi.ERRNO_BADF;
            }

            const iovecs = wasi.Ciovec.read_bytes_array(
                dataView(),
                iovs_ptr,
                iovs_len,
            );
            let totalWritten = 0;
            for (const iovec of iovecs) {
                // const buffer = new Uint8Array(iovec.buf);
                // uint8().set(buffer, iovec.buf);
                const { ret, nwritten: written } = handle.fd_write(
                    uint8().slice(iovec.buf, iovec.buf + iovec.buf_len),
                );
                if (ret !== wasi.ERRNO_SUCCESS) {
                    return ret;
                }
                totalWritten += written;
                if (written !== iovec.buf_len) {
                    break;
                }
            }
            dataView().setUint32(nwritten_ptr, totalWritten, true);
            return wasi.ERRNO_SUCCESS;
        },
        /**
         * The path_create_directory() function creates a new directory at the specified path relative to the given directory.
         * It requires the PATH_CREATE_DIRECTORY right to be set on the directory where the new directory will be created.
         *
         * On POSIX systems, a similar functionality is provided by the mkdir() function.
         * It creates a new directory with the given name and the specified permission mode.
         * The mkdir() function is part of the POSIX standard and is widely supported across different platforms.
         */
        path_create_directory(
            fd: number,
            path_ptr: number,
            path_len: number,
        ): number {
            const handle = fds[fd];
            if (!handle) {
                return wasi.ERRNO_BADF;
            }
            const path = new TextDecoder("utf-8").decode(
                uint8().slice(path_ptr, path_ptr + path_len),
            );
            console.log("path_create_directory", path);
            return handle.path_create_directory(path);
        },
        /**
         * The path_filestat_get() function allows accessing metadata (file statistics) for a file or directory specified by a path relative to the given directory.
         * It retrieves information such as the size, timestamps, and file type.
         *
         * On POSIX systems, a similar functionality is provided by the stat() or lstat() functions, depending on whether symbolic links should be followed or not.
         * These functions retrieve information about a file or symbolic link and store it in a struct stat object.
         */
        async path_filestat_get(
            fd: number,
            flags: number,
            path_ptr: number,
            path_len: number,
            filestat_ptr: number,
        ): Promise<number> {
            const handle = fds[fd];
            if (!handle) {
                return wasi.ERRNO_BADF;
            }
            const path = new TextDecoder("utf-8").decode(
                uint8().slice(path_ptr, path_ptr + path_len),
            );
            const { ret, filestat } = handle.path_filestat_get(flags, path);
            if (filestat != null) {
                filestat.write_bytes(dataView(), filestat_ptr);
            }
            return ret;
        },
        /**
         * The path_filestat_set_times() function allows updating the time metadata (last accessed time and last modified time) for a file or directory specified by a path relative to the given directory.
         *
         * On POSIX systems, a similar functionality is provided by the utimensat() function.
         * It updates the timestamps (access time and modification time) of a file or directory with nanosecond precision.
         */
        path_filestat_set_times(
            _fd: number,
            _flags: number,
            _path_ptr: number,
            _path_len: number,
            _atim: number, // bigint,
            _mtim: number, // bigint,
            _fst_flags: number,
        ) {
            return wasi.ERRNO_NOTSUP;
        },
        /**
         * The path_link() function creates a hard link between two files.
         * It creates a new directory entry with the specified name in the destination directory, which refers to the same underlying file as the source file.
         *
         * On POSIX systems, a similar functionality is provided by the link() function.
         * It creates a new link (directory entry) for an existing file.
         * The new link and the original file refer to the same inode and share the same content.
         */
        path_link(
            _old_fd: number,
            _old_flags: number,
            _old_path_ptr: number,
            _old_path_len: number,
            _new_fd: number,
            _new_path_ptr: number,
            _new_path_len: number,
        ): number {
            return wasi.ERRNO_NOTSUP;
        },
        /**
         * The path_open() function opens a file or directory at the specified path relative to the given directory.
         * It provides various options for how the file will be opened, including read and write access, creation flags,
         * and file descriptor flags.
         *
         * On POSIX systems, a similar functionality is provided by the open() function.
         * It opens a file or directory with the specified flags and mode.
         * The open() function is a widely used system call for file operations in POSIX-compliant operating systems.
         */
        path_open(
            fd: number,
            dirflags: number,
            path_ptr: number,
            path_len: number,
            oflags: number, // Unsupported
            fs_rights_base: number, // bigint
            fs_rights_inheriting: number, // bigint
            fd_flags: number, // Unsupported
            opened_fd_ptr: number,
        ): number {
            const handle = fds[fd];
            if (!handle) {
                return wasi.ERRNO_BADF;
            }
            const path = new TextDecoder("utf-8").decode(
                uint8().slice(path_ptr, path_ptr + path_len),
            );
            const { ret, fd_obj } = handle.path_open(
                dirflags,
                path,
                oflags,
                BigInt(fs_rights_base),
                BigInt(fs_rights_inheriting),
                fd_flags,
            );
            if (ret != 0) {
                return ret;
            }
            if (!fd_obj) {
                throw new Error("fd_obj is null");
            }
            fds.push(fd_obj);
            const opened_fd = fds.length - 1;
            dataView().setUint32(opened_fd_ptr, opened_fd, true);
            return wasi.ERRNO_SUCCESS;
        },
        /**
         * The path_readlink() function reads the target path that a symlink points to.
         * It requires the PATH_READLINK right to be set on the base directory from which the symlink is understood.
         *
         * On POSIX systems, a similar functionality is provided by the readlink() function.
         * It reads the value of a symbolic link and stores it in a buffer.
         * The readlink() function is part of the POSIX standard and is widely supported across different platforms.
         */
        path_readlink(
            _fd: number,
            _path_ptr: number,
            _path_len: number,
            _buf_ptr: number,
            _buf_len: number,
            _nread_ptr: number,
        ): number {
            return wasi.ERRNO_NOTSUP;
        },
        /**
         * The path_remove_directory() function removes a directory specified by the given path.
         * It requires the PATH_REMOVE_DIRECTORY right to be set on the directory.
         *
         * On POSIX systems, a similar functionality is provided by the rmdir() function.
         * It removes an empty directory with the specified path.
         * The rmdir() function is part of the POSIX standard and is widely supported across different platforms.
         */
        path_remove_directory(
            fd: number,
            path_ptr: number,
            path_len: number,
        ): number {
            const handle = fds[fd];
            if (!handle) {
                return wasi.ERRNO_BADF;
            }
            const path = new TextDecoder("utf-8").decode(
                uint8().slice(path_ptr, path_ptr + path_len),
            );
            return handle.path_remove_directory(path);
        },
        /**
         * The path_rename() function renames a file or directory specified by the given path.
         * It requires the PATH_RENAME_SOURCE right on the source directory and the PATH_RENAME_TARGET right on the target directory.
         *
         * On POSIX systems, a similar functionality is provided by the rename() function.
         * It renames a file or directory with the specified source and target paths.
         * The rename() function is part of the POSIX standard and is widely supported across different platforms.
         */
        path_rename(
            _fd: number,
            _old_path_ptr: number,
            _old_path_len: number,
            _new_fd: number,
            _new_path_ptr: number,
            _new_path_len: number,
        ): number {
            return wasi.ERRNO_NOTSUP;
        },
        /**
         * The path_symlink() function creates a symbolic link (symlink) with the specified source path pointing to the target path.
         * It requires the PATH_SYMLINK right on the base directory.
         *
         * On POSIX systems, a similar functionality is provided by the symlink() function.
         * It creates a symbolic link with the specified source and target paths.
         * The symlink() function is part of the POSIX standard and is widely supported across different platforms.
         */
        path_symlink(
            _old_path_ptr: number,
            _old_path_len: number,
            _fd: number,
            _new_path_ptr: number,
            _new_path_len: number,
        ): number {
            return wasi.ERRNO_NOTSUP;
        },
        /**
         * The path_unlink_file() function unlinks a file at the specified path.
         * If the file has only one hardlink (i.e., its link count is 1), it will be deleted from the file system.
         * It requires the PATH_UNLINK_FILE right on the base file descriptor.
         *
         * On POSIX systems, a similar functionality is provided by the unlink() function.
         * It removes the specified file from the file system.
         * If the file has no other hardlinks, it is completely deleted.
         * The unlink() function is part of the POSIX standard and is widely supported across different platforms.
         */
        path_unlink_file(
            _fd: number,
            _path_ptr: number,
            _path_len: number,
        ): number {
            return wasi.ERRNO_NOTSUP;
        },
    };

    return Object.values(implement).reduce((acc, func, index) => {
        acc[index] = func;
        return acc;
    }, {} as Record<number, (...args: number[]) => number | void | Promise<number> | Promise<void>>);
}
