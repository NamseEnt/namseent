import { ConsoleStdout } from "@bjorn3/browser_wasi_shim";

export function stdout(threadId: number) {
    return ConsoleStdout.lineBuffered((msg) => {
        console.log(`[${threadId}] ${msg}`);
    });
}
