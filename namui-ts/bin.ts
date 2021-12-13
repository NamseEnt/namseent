import { Command } from "commander";
import { build } from "./buildServer/build";
const program = new Command();

program
  .argument("<entryPoint>", "file to build")
  .option("--watch", "watch", false);

program.parse();

const [entryPoint] = program.args;
const { watch } = program.opts();

if (!entryPoint) {
  throw new Error("entry point is required");
}

build({
  entryPoint,
  watch,
});
