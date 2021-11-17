import { toServerRpcHandler } from "./toServerRpcHandler";
import fs from "fs/promises";
import path from "path";
import { resourcesRoot } from "./resourcesRoot";

export const onReadFile: typeof toServerRpcHandler["onReadFile"] = async (
  context,
  { destPath },
) => {
  const destPathAbsolute = path.join(resourcesRoot, destPath);
  const data = await fs.readFile(destPathAbsolute);
  return data;
};
