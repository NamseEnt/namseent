declare var CanvasKitInit: any;

export async function initCanvasKit() {
    await loadCanvasKit();

    const [CanvasKit] = await Promise.all([
        CanvasKitInit({
            locateFile: (file: string) => "./canvaskit-wasm/" + file,
        }),
    ]);

    (globalThis as any).CanvasKit = CanvasKit;
    (globalThis as any).getCanvasKit = () => CanvasKit;
}

async function loadCanvasKit(): Promise<void> {
    if ("importScripts" in globalThis) {
        return importScripts("./canvaskit-wasm/canvaskit.js");
    }
    return new Promise((resolve, reject) => {
        const script = document.createElement("script");
        script.src = "./canvaskit-wasm/canvaskit.js";
        script.onload = () => {
            resolve();
        };
        script.onerror = () => {
            reject();
        };
        document.body.appendChild(script);
    });
}
