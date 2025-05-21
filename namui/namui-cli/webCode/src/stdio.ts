import {
    ConsoleStdout,
} from "@bjorn3/browser_wasi_shim";
import { sendMessageToMainThread } from "./interWorkerProtocol";

export function stdout(threadId: number) {
    return ConsoleStdout.lineBuffered((msg) =>
    {
        if (msg.startsWith("\u0081")) {
            sendMessageToMainThread({
                type: "log",
                threadId,
                msg: msg.slice(1),
            });
        } else {
            console.log(`[${threadId}] ${msg}`);
        }
    });
}
