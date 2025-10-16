import { defineConfig } from "vite";
import expressPlugin from "./expressPlugin";
import { assetCollectorPlugin } from "./assetCollectorPlugin";
import { namuiHmrPlugin } from "./namuiHmrPlugin";
import path from "path";

// Read configuration from environment variables
const namuiBundleSqlitePath = process.env.NAMUI_BUNDLE_SQLITE_PATH;
const namuiDrawerWasmPath = process.env.NAMUI_DRAWER_WASM_PATH;
const namuiHost = process.env.NAMUI_HOST || "localhost";
const namuiAssetDir = process.env.NAMUI_ASSET_DIR;

// Parse allow and fs.allow arrays from environment variables
const serverAllow = process.env.NAMUI_SERVER_ALLOW
    ? JSON.parse(process.env.NAMUI_SERVER_ALLOW)
    : [];
const serverFsAllow = process.env.NAMUI_SERVER_FS_ALLOW
    ? JSON.parse(process.env.NAMUI_SERVER_FS_ALLOW)
    : ["./"];

export default defineConfig({
    clearScreen: false,
    server: {
        headers: {
            "Cross-Origin-Resource-Policy": "same-origin",
            "Cross-Origin-Embedder-Policy": "require-corp",
            "Cross-Origin-Opener-Policy": "same-origin",
            "Referrer-Policy": "no-referrer-when-downgrade",
        },
        allow: serverAllow,
        fs: {
            allow: serverFsAllow,
        },
        host: namuiHost,
    },
    resolve: {
        alias: {
            "bundle.sqlite?url": `${namuiBundleSqlitePath}?url`,
            "namui-drawer.wasm?url": `${namuiDrawerWasmPath}?url`,
            "@": path.resolve(__dirname, "./src"),
        },
    },
    plugins: [
        namuiHmrPlugin(),
        expressPlugin(),
        assetCollectorPlugin(namuiAssetDir),
    ],
});
