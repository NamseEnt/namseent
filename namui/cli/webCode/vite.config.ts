import { defineConfig } from "vite";
import { wasmPlugin } from "./wasmPlugin";

export default defineConfig({
    clearScreen: false,
    plugins: [wasmPlugin()],
    server: {
        headers: {
            "Cross-Origin-Resource-Policy": "same-origin",
            "Cross-Origin-Embedder-Policy": "require-corp",
            "Cross-Origin-Opener-Policy": "same-origin",
            "Referrer-Policy": "no-referrer-when-downgrade",
        },
        host: "localhost",
    },
});
