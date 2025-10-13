import { startThread } from "./thread/startThread";
import drawerUrl from "namui-drawer.wasm?url";

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

console.log("instance.exports", instance.exports);
console.log("before test");
(instance.exports as any)._test();
console.log("after test");

const instance2 = await startThread({
    type: "drawer",
    memory,
    module,
    nextTid,
    initialWindowWh: (window.innerWidth << 16) | window.innerHeight,
});

(instance2.exports as any)._test();
(instance.exports as any)._test();
