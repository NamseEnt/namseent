import { ViteDevServer } from "vite";
import { watch } from "fs/promises";
import { existsSync } from "fs";

export function namuiHmrPlugin() {
    const wasmPath = process.env.NAMUI_RUNTIME_WASM_PATH;

    console.log("wasmPath", wasmPath);
    if (!wasmPath) {
        throw new Error(
            "NAMUI_RUNTIME_WASM_PATH environment variable is not set",
        );
    }

    const VIRTUAL_MODULE_ID = "virtual:namui-runtime-wasm.wasm?url";
    const RESOLVED_VIRTUAL_MODULE_ID = "\0" + VIRTUAL_MODULE_ID;

    return {
        name: "namui-hmr-plugin",
        resolveId(id: string) {
            if (id === VIRTUAL_MODULE_ID) {
                return RESOLVED_VIRTUAL_MODULE_ID;
            }
        },
        load(id: string) {
            if (id === RESOLVED_VIRTUAL_MODULE_ID) {
                return `export default "/@fs${wasmPath}"`;
            }
        },
        configureServer(server: ViteDevServer) {
            (async () => {
                try {
                    const watcher = watch(wasmPath);
                    for await (const event of watcher) {
                        if (
                            event.eventType === "change" ||
                            (event.eventType === "rename" &&
                                existsSync(wasmPath))
                        ) {
                            server.hot.send("namui-wasm-updated", { wasmPath });
                        }
                    }
                } catch (err) {
                    console.error("Error watching wasm file:", err);
                }
            })();
        },
    };
}
