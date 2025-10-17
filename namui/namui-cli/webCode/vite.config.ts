import { defineConfig } from "vite";
import expressPlugin from "./expressPlugin";
import { assetCollectorPlugin } from "./assetCollectorPlugin";
import { namuiPlugin } from "./namuiPlugin";
import path from "path";

if (!process.env.NAMUI_APP_PATH) {
    throw new Error("NAMUI_APP_PATH is not defined");
}
const { NAMUI_APP_PATH } = process.env;

const serverFsAllow = ["./"];

export default defineConfig({
    clearScreen: false,
    server: {
        headers: {
            "Cross-Origin-Resource-Policy": "same-origin",
            "Cross-Origin-Embedder-Policy": "require-corp",
            "Cross-Origin-Opener-Policy": "same-origin",
            "Referrer-Policy": "no-referrer-when-downgrade",
        },
        fs: {
            allow: serverFsAllow,
        },
    },
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./src"),
        },
    },
    plugins: [
        namuiPlugin(),
        expressPlugin(),
        assetCollectorPlugin(path.join(NAMUI_APP_PATH, "asset")),
    ],
});
