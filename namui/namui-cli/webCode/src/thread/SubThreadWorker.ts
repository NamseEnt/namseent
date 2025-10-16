import { startThread } from "./startThread";

self.onmessage = async (message) => {
    startThread(message.data);
};
