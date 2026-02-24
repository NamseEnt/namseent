import { startThread } from "./startThread";

self.onmessage = async (message) => {
    try {
        await startThread(message.data);
    } catch (e) {
        console.error("[SubThreadWorker] error:", e);
    }
};
