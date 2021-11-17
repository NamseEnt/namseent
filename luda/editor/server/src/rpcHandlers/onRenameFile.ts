import { toServerRpcHandler } from "./toServerRpcHandler";
import fs from "fs/promises";
import path from "path";
import { resourcesRoot } from "./resourcesRoot";

export const onRenameFile: typeof toServerRpcHandler["onRenameFile"] = async (
  context,
  { oldPath, newPath },
) => {
  const oldPathAbsolute = path.join(resourcesRoot, oldPath);
  const newPathAbsolute = path.join(resourcesRoot, newPath);

  await fs.rename(oldPathAbsolute, newPathAbsolute);

  return;
};
