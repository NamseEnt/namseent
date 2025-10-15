import FontLoadWorker from "./FontLoadWorker?worker";

export function loadFonts({
    memory,
    module,
}: {
    memory: WebAssembly.Memory;
    module: WebAssembly.Module;
}): Promise<void> {
    const worker = new FontLoadWorker();
    return new Promise((resolve, reject) => {
        worker.onmessage = () => {
            resolve();
        };
        worker.onerror = (error) => {
            reject(error);
        };
        worker.postMessage({
            memory,
            module,
        });
    });
}
