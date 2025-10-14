import { startThread } from "../thread/startThread";
import drawerUrl from "namui-drawer.wasm?url";
import { assetList } from "virtual:asset-list";
import { DrawerExports } from "./types";
import { processImages } from "./imageLoader";

const memory = new WebAssembly.Memory({
    initial: 128,
    maximum: 16384,
    shared: true,
});

const nextTid = new SharedArrayBuffer(4);
new Uint32Array(nextTid)[0] = 1;

const module = await WebAssembly.compileStreaming(fetch(drawerUrl));

const instance = await startThread({
    type: "drawer",
    memory,
    module,
    nextTid,
    initialWindowWh: (window.innerWidth << 16) | window.innerHeight,
});

console.log("Main drawer instance created");
console.log("instance.exports", instance.exports);

const startTime = performance.now();
await processImages(assetList, instance.exports as DrawerExports, memory);

console.log(
    `All images loaded successfully, ${(performance.now() - startTime).toFixed(
        2,
    )}ms`,
);
