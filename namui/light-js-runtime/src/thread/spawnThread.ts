import type { ThreadStartSupplies } from "./threadMain";
import ThreadWorker from "./ThreadWorker?worker";

export function spawnThread(supplies: ThreadStartSupplies) {
    const worker = new ThreadWorker();
    worker.postMessage(supplies);
}
