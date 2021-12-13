import { Plugin } from "esbuild";

export const printBuildTimePlugin: Plugin = {
  name: "printBuildTime",
  setup: (build) => {
    let startTime: number;

    build.onStart(() => {
      startTime = Date.now();
    });

    build.onEnd(() => {
      console.log(
        `[${new Date().toLocaleTimeString()}] 🚀 build done. (${
          (Date.now() - startTime) / 1000
        }s)`,
      );
    });
  },
};
