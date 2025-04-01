import express from "express";
import { ViteDevServer } from "vite";
import compression from "compression";

const app = express();
app.use(compression({}));

export default function expressPlugin() {
    return {
        name: "express-plugin",
        configureServer(server: ViteDevServer) {
            server.middlewares.use(app);
        },
    };
}
