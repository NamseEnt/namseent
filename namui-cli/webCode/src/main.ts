import { cacheGet, cacheSet } from "./cache.js";
import { runMessageLoop } from "./messageLoop.js";

const workerToMainBufferSab = new SharedArrayBuffer(16 * 1024 * 1024);
const mainToWorkerBufferSab = new SharedArrayBuffer(16 * 1024 * 1024);

runMessageLoop(workerToMainBufferSab, (message) => {
    switch (message.type) {
        case "getBaseUrl": {
            return {
                baseUrl: window.document.URL,
            };
        }
    }
});

const myWorker = new Worker("worker.js", {
    type: "classic",
});

myWorker.postMessage({
    type: "init",
    workerToMainBufferSab,
    mainToWorkerBufferSab,
});

myWorker.onerror = (e) => {
    console.error(e, "error on worker");
};
myWorker.onmessage = ({ data }) => {
    console.log("message from worker", data);
    switch (data.type) {
        case "cacheGet": {
            const { key } = data;
            const value = cacheGet(key);
            myWorker.postMessage({
                type: "cacheGet",
                key,
                value,
            });
            break;
        }
        case "cacheSet": {
            const { key, value } = data;
            cacheSet(key, value);
            myWorker.postMessage({
                type: "cacheSet",
                key,
                value,
            });
            break;
        }
    }
};
myWorker.onmessageerror = (e) => {
    console.log("message error from worker", e);
};
