import typescript from "@rollup/plugin-typescript";
import resolve from "@rollup/plugin-node-resolve";

/** @type {import('rollup').RollupOptions} */
const defaultConfig = {
    output: {
        dir: "../www",
        format: "iife",
        sourcemap: "inline",
    },
    plugins: [typescript(), resolve()],
};

const inputs = ["src/main.ts", "src/worker.ts"];
export default inputs.map((input) => ({
    ...defaultConfig,
    input,
}));
