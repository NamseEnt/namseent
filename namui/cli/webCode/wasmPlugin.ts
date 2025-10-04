import { Plugin } from "vite";
import * as fs from "fs";

export function wasmPlugin(): Plugin {
    const wasmPath = process.env.WASM_PATH;

    if (!wasmPath) {
        throw new Error("WASM_PATH environment variable is not set");
    }

    const virtualWasmUrl = "/bundle.wasm";

    return {
        name: "wasm-plugin",
        resolveId(id) {
            if (id === "virtual:wasm") {
                return id;
            }
        },
        load(id) {
            if (id === "virtual:wasm") {
                return `export default "${virtualWasmUrl}";`;
            }
        },
        configureServer(server) {
            // Serve wasm file
            server.middlewares.use((req, res, next) => {
                if (req.url?.startsWith(virtualWasmUrl)) {
                    if (fs.existsSync(wasmPath)) {
                        res.setHeader('Content-Type', 'application/wasm');
                        res.setHeader('Cache-Control', 'no-cache');
                        fs.createReadStream(wasmPath).pipe(res);
                    } else {
                        res.statusCode = 404;
                        res.end('WASM file not found');
                    }
                } else {
                    next();
                }
            });

            // Watch wasm file changes (직접 감시, vite watcher 사용 안 함)
            if (fs.existsSync(wasmPath)) {
                let lastMtime = fs.statSync(wasmPath).mtimeMs;

                const checkInterval = setInterval(() => {
                    try {
                        const currentMtime = fs.statSync(wasmPath).mtimeMs;
                        if (currentMtime > lastMtime) {
                            lastMtime = currentMtime;
                            console.log('WASM file changed, triggering HMR');
                            server.ws.send({
                                type: 'custom',
                                event: 'wasm-update',
                                data: { path: virtualWasmUrl, timestamp: Date.now() }
                            });
                        }
                    } catch (err) {
                        // File might not exist temporarily during build
                    }
                }, 100);

                // Cleanup on server close
                server.httpServer?.on('close', () => clearInterval(checkInterval));
            }
        },
    };
}
