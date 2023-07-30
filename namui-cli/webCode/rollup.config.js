import typescript from "@rollup/plugin-typescript";

/** @type {import('rollup').RollupOptions} */
const defaultConfig = {
    output: {
        dir: "../www",
        format: "umd",
        sourcemap: "inline",
    },
    plugins: [typescript()],
};

const inputs = ["src/main.ts", "src/worker.ts"];
export default inputs.map((input) => ({
    ...defaultConfig,
    input,
}));
