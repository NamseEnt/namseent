import typescript from "@rollup/plugin-typescript";
import resolve from "@rollup/plugin-node-resolve";
import { babel } from "@rollup/plugin-babel";
import nodePolyfills from "rollup-plugin-polyfill-node";
import copy from "rollup-plugin-copy";

/** @type {import('rollup').RollupOptions} */
const defaultConfig = {
    output: {
        dir: "../www",
        format: "iife",
        sourcemap: true,
        intro: `const NAMUI_ENV = "${process.env.NAMUI_ENV}";`,
    },
    plugins: [
        nodePolyfills(),
        typescript({
            sourceMap: true,
            tsconfig: "./tsconfig.json",
        }),
        resolve({
            browser: true,
        }),
        babel({
            babelHelpers: "bundled",
            sourceMaps: true,
        }),
        copy({
            targets: [
                {
                    src: "node_modules/canvaskit-wasm/bin/*",
                    dest: "../www/canvaskit-wasm",
                },
            ],
        }),
    ],
};

const inputs = ["src/main/main.ts", "src/worker.ts"];
export default inputs.map((input) => ({
    ...defaultConfig,
    input,
}));
