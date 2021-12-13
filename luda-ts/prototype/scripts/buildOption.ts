import { BuildOptions } from "esbuild";
import path from "path";
import { copyIndexHtml } from "./esbuildPlugin/copyIndexHtml";
import { printBuildTimePlugin } from "./esbuildPlugin/printBuildTime";

export const outdir = path.join(__dirname, "../dist");

export const buildOption: BuildOptions = {
  entryPoints: [path.join(__dirname, "../src/index.tsx")],
  outdir,
  bundle: true,
  minify: true,
  sourcemap: false,
  platform: "browser",
  loader: {
    ".mp3": "file",
    ".jpg": "file",
    ".png": "file",
    ".wav": "file",
    ".webm": "file",
  },
  logLevel: "info",
  plugins: [printBuildTimePlugin, copyIndexHtml],
};
