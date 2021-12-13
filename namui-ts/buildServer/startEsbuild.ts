import * as esbuild from "esbuild";
import { ErrorMessage } from "../src/build/type";

export async function startEsbuild({
  entryPoint,
  onRebuild,
}: {
  entryPoint: string;
  onRebuild: (errors: ErrorMessage[]) => void;
}): Promise<{
  getBuild: () => string;
}> {
  const onBuild = (
    buildFailure: esbuild.BuildFailure | null,
    result: esbuild.BuildResult | null,
  ) => {
    if (result?.outputFiles && result.outputFiles[0]) {
      build = result.outputFiles[0].text;
    }
    const errorMessages: ErrorMessage[] = buildFailure
      ? [...buildFailure.warnings, ...buildFailure.errors]
          .filter((x) => x.location)
          .map((x) => {
            return {
              absoluteFile: x.location!.file,
              relativeFile: x.location!.file,
              column: x.location!.column,
              line: x.location!.line,
              text: x.text,
            };
          })
      : [];
    onRebuild(errorMessages);
  };

  let build: string;
  let buildResult: esbuild.BuildResult | undefined = undefined;
  try {
    buildResult = await esbuild.build({
      watch: {
        onRebuild: onBuild,
      },
      bundle: true,
      external: ["path", "fs"],
      write: false,
      sourcemap: true,
      entryPoints: [entryPoint],
      logLevel: "silent",
    });
  } catch (error) {
    const failure = error as esbuild.BuildFailure;
    onBuild(failure, null);
  }

  if (buildResult) {
    onBuild(null, buildResult);
  }

  const getBuild = () => build;

  return {
    getBuild,
  };
}
