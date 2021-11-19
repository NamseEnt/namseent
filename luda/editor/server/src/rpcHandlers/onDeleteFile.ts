import { toServerRpcHandler } from "./toServerRpcHandler";
import fs from "fs/promises";
import path from "path";
import { resourcesRoot } from "./resourcesRoot";

export const onRemoveFile: typeof toServerRpcHandler["onRemoveFile"] = async (
  context,
  { destPath },
) => {
  const destPathAbsolute = path.join(resourcesRoot, destPath);
  await fs.rm(destPathAbsolute);
  return;
};
