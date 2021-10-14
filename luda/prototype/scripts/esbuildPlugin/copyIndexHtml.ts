import { Plugin } from "esbuild";
import fs from "fs-extra";
import path from "path";
import { outdir } from "../buildOption";

export const copyIndexHtml: Plugin = {
  name: "copyIndexHtml",
  setup: (build) => {
    build.onStart(async () => {
      await fs.copy(
        path.join(__dirname, "../../public/index.html"),
        path.join(outdir, "index.html"),
        {
          overwrite: true,
        },
      );
    });
  },
};
