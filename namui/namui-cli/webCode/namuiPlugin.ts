import { ViteDevServer } from "vite";
import { watch } from "fs/promises";
import path from "path";

if (!process.env.NAMUI_APP_PATH) {
    throw new Error("NAMUI_APP_PATH is not defined");
}
const { NAMUI_APP_PATH } = process.env;

export function namuiPlugin() {
    const appWasmPath = path.join(
        NAMUI_APP_PATH,
        "target/namui/target/wasm32-wasip1-threads/debug/namui-runtime-wasm.wasm",
    );
    const drawerWasmPath = path.join(
        __dirname,
        "../../namui-drawer/target/wasm32-wasip1-threads/release/namui-drawer.wasm",
    );

    const VIRTUAL_MODULE_ID_APP = "virtual:namui-runtime-wasm.wasm?url";
    const VIRTUAL_MODULE_ID_DRAWER = "virtual:namui-drawer.wasm?url";
    const RESOLVED_VIRTUAL_MODULE_ID_APP = "\0" + VIRTUAL_MODULE_ID_APP;
    const RESOLVED_VIRTUAL_MODULE_ID_DRAWER = "\0" + VIRTUAL_MODULE_ID_DRAWER;

    const appWatcher = new Watcher(appWasmPath);
    const drawerWatcher = new Watcher(drawerWasmPath);

    return {
        name: "namui-hmr-plugin",
        resolveId(id: string) {
            if (id === VIRTUAL_MODULE_ID_APP) {
                return RESOLVED_VIRTUAL_MODULE_ID_APP;
            }
            if (id === VIRTUAL_MODULE_ID_DRAWER) {
                return RESOLVED_VIRTUAL_MODULE_ID_DRAWER;
            }
        },
        load(id: string) {
            if (id === RESOLVED_VIRTUAL_MODULE_ID_APP) {
                return `export default "/@fs${appWasmPath}"`;
            }
            if (id === RESOLVED_VIRTUAL_MODULE_ID_DRAWER) {
                return `export default "/@fs${drawerWasmPath}"`;
            }
        },
        configureServer(server: ViteDevServer) {
            appWatcher.onConfigureServer(server, () => {
                server.hot.send("namui-app-wasm-updated");
            });
            drawerWatcher.onConfigureServer(server, () => {
                server.hot.send("namui-drawer-wasm-updated");
            });
        },
    };
}

class Watcher {
    killLastWatch: () => void = () => {};
    constructor(private readonly watchingfilePath: string) {}

    async onConfigureServer(server: ViteDevServer, onFileUpdated: () => void) {
        this.killLastWatch();

        let killed = false;
        this.killLastWatch = () => {
            killed = true;
        };

        try {
            let id: NodeJS.Timeout | undefined;
            const watcher = watch(path.resolve(this.watchingfilePath, "../"));
            for await (const event of watcher) {
                if (killed) {
                    return;
                }
                if (
                    event.eventType === "change" &&
                    event.filename === path.basename(this.watchingfilePath)
                ) {
                    clearTimeout(id);
                    id = setTimeout(() => {
                        onFileUpdated();
                    }, 500);
                }
            }
            console.log("watcher done");
        } catch (err) {
            console.error("Error watching wasm file:", err);
        }
    }
}
