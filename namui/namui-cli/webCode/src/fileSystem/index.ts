import { WASI } from "@bjorn3/browser_wasi_shim";
import { sendMessageToMainThread } from "../interWorkerProtocol";

export function overrideWasiFs({
    wasi,
    threadId,
}: {
    wasi: WASI;
    threadId: number;
}) {
    const protocolBuffer = new SharedArrayBuffer(128);
    sendMessageToMainThread({
        type: "fs-thread-connect",
        protocolBuffer,
        threadId,
    });
    const int32 = new Int32Array(protocolBuffer);

    function requestToFsWorker(...args: number[]): number {
        const noLog = !(
            fsFuncs[args[0]] === "fd_write" && [1, 2].includes(args[1])
        );

        const argsCount = args.length;
        int32[0] = argsCount;
        int32.set(args, 1);
        Atomics.notify(int32, 0);

        Atomics.wait(int32, 0, argsCount);
        const response = Atomics.load(int32, 1);
        noLog &&
            console.log(
                `response of function ${
                    fsFuncs[args[0] as number]
                } : ${response}`,
            );
        return response;
    }

    fsFuncs.forEach((name, index) => {
        wasi.wasiImport[name] = (...args) => {
            return requestToFsWorker(index, ...args.map((x) => Number(x)));
        };
    });
}

export const fsFuncs = [
    "fd_advise",
    "fd_allocate",
    "fd_close",
    "fd_datasync",
    "fd_fdstat_get",
    "fd_fdstat_set_flags",
    "fd_fdstat_set_rights",
    "fd_filestat_get",
    "fd_filestat_set_size",
    "fd_filestat_set_times",
    "fd_pread",
    "fd_prestat_get",
    "fd_prestat_dir_name",
    "fd_pwrite",
    "fd_read",
    "fd_readdir",
    "fd_renumber",
    "fd_seek",
    "fd_sync",
    "fd_tell",
    "fd_write",
    "path_create_directory",
    "path_filestat_get",
    "path_filestat_set_times",
    "path_link",
    "path_open",
    "path_readlink",
    "path_remove_directory",
    "path_rename",
    "path_symlink",
    "path_unlink_file",
];
