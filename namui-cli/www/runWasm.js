async function run() {
    await wasm_bindgen("/bundle_bg.wasm");
    const CanvasKit = await CanvasKitInit({
        locateFile: (file) => "./canvaskit-wasm/" + file,
    });
    globalThis.CanvasKit = CanvasKit;
    globalThis.getCanvasKit = () => CanvasKit;
    wasm_bindgen.start();
}

run();
