export {};

const root = await navigator.storage.getDirectory();
const kvDir = await root.getDirectoryHandle("kv_store", { create: true });

self.onmessage = async (e: MessageEvent) => {
    const { requestId, op, key, value } = e.data;

    try {
        if (op === "get") {
            try {
                const fileHandle = await kvDir.getFileHandle(key);
                const file = await fileHandle.getFile();
                const data = new Uint8Array(await file.arrayBuffer());
                self.postMessage({ requestId, op, hasData: true, data });
            } catch {
                self.postMessage({ requestId, op, hasData: false });
            }
        } else if (op === "put") {
            const fileHandle = await kvDir.getFileHandle(key, { create: true });
            const writable = await fileHandle.createWritable();
            await writable.write(value);
            await writable.close();
            self.postMessage({ requestId, op });
        } else if (op === "delete") {
            try {
                await kvDir.removeEntry(key);
            } catch {
                // ignore if not exists
            }
            self.postMessage({ requestId, op });
        }
    } catch (error) {
        console.error("[StorageWorker] error:", error);
        if (op === "get") {
            self.postMessage({ requestId, op, hasData: false });
        } else {
            self.postMessage({ requestId, op });
        }
    }
};
