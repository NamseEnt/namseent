import path from "path";
import fs from "fs/promises";

export type NamuiConfig = {
  // extends by engine
  resources?: string;
  // set by engine
  rootDirectoryPath: string;
  // fallback by engine
  tsconfigFilePath: string;
};

export async function getNamuiConfig(): Promise<NamuiConfig> {
  const rootDirectoryPath = getProjectRootDirectoryPath();
  const namuiConfigPath = path.join(rootDirectoryPath, "namui-config.json");
  const namuiConfigString = await fs.readFile(namuiConfigPath, "utf8");
  const namuiConfig = JSON.parse(namuiConfigString) as NamuiConfig;

  if (namuiConfig.resources) {
    namuiConfig.resources = path.join(rootDirectoryPath, namuiConfig.resources);
  }
  namuiConfig.rootDirectoryPath = rootDirectoryPath;
  namuiConfig.tsconfigFilePath ??= path.join(
    rootDirectoryPath,
    "tsconfig.json",
  );

  return namuiConfig;
}

function getProjectRootDirectoryPath(): string {
  return process.cwd();
}
