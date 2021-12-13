import * as wasm from "hello_wasm";

(async () => {
  const CanvasKit = await CanvasKitInit({
    locateFile: (file) => "./canvaskit-wasm/" + file,
  });
  globalThis.CanvasKit = CanvasKit;
  globalThis.getCanvasKit = () => CanvasKit;
  wasm.start();
})();
