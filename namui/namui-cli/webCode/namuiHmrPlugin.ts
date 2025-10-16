import { ViteDevServer } from "vite";
import { watch } from "fs/promises";
import path from "path";

let configureServerUpdated = () => {};

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
            configureServerUpdated();

            (async (
                registerUpdatedCallback: (callback: () => void) => void,
            ) => {
                let updated = false;
                registerUpdatedCallback(() => {
                    updated = true;
                });

                try {
                    let id: NodeJS.Timeout | undefined;
                    const watcher = watch(path.resolve(wasmPath, "../"));
                    for await (const event of watcher) {
                        if (updated) {
                            return;
                        }
                        if (
                            event.eventType === "change" &&
                            event.filename === "namui-runtime-wasm.wasm"
                        ) {
                            clearTimeout(id);
                            id = setTimeout(() => {
                                server.hot.send("namui-wasm-updated", {
                                    wasmPath,
                                });
                            }, 500);
                        }
                    }
                    console.log("watcher done");
                } catch (err) {
                    console.error("Error watching wasm file:", err);
                }
            })((callback) => {
                configureServerUpdated = callback;
            });
        },
    };
}
