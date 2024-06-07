import {
    ConsoleStdout,
    Fd,
    File,
    Inode,
    OpenFile,
    PreopenDirectory,
    Directory,
} from "@bjorn3/browser_wasi_shim";

export function getFds(bundleSharedTree: BundleSharedTree): Fd[] {
    return [
        new OpenFile(new File([])), // stdin
        ConsoleStdout.lineBuffered((msg) =>
            console.log(`[WASI stdout] ${msg}`),
        ),
        ConsoleStdout.lineBuffered((msg) =>
            console.warn(`[WASI stderr] ${msg}`),
        ),
        new PreopenDirectory("bundle", makeDictionaryTree(bundleSharedTree)),
    ];
}

export type BundleSharedTree = {
    [key: string]: BundleSharedTree | SharedArrayBuffer;
};

function makeDictionaryTree(tree: BundleSharedTree): Map<string, Inode> {
    const contents = new Map<string, Inode>();
    Object.entries(tree).map(([key, value]) => {
        if (value instanceof SharedArrayBuffer) {
            contents.set(
                key,
                new File(new Uint8Array(value), { readonly: true }),
            );
        } else {
            contents.set(key, new Directory(makeDictionaryTree(value)));
        }
    });
    return contents;
}
