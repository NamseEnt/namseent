import { defineConfig } from "vite";

export default defineConfig({
    clearScreen: false,
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
