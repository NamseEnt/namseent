import * as wasm from "hello_wasm";

window.test = (canvas) => {
  console.log(canvas);
}

(async () => {
  const CanvasKit = await CanvasKitInit({
    locateFile: (file) => "./canvaskit-wasm/" + file,
  });
  globalThis.CanvasKit = CanvasKit;
  window.CanvasKit = CanvasKit;
  console.log(CanvasKit);

  wasm.greet();
})();
