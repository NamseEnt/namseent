import { toServerRpcHandler } from "./toServerRpcHandler";
import fs from "fs/promises";
import path from "path";
import { resourcesRoot } from "./resourcesRoot";
import { Dirent } from "luda-editor-common";

export const onListFiles: typeof toServerRpcHandler["onListFiles"] = async (
  context,
  { directoryPath },
) => {
  const directoryPathAbsolute = path.join(resourcesRoot, directoryPath);
  const dirents = await fs.readdir(directoryPathAbsolute, {
    withFileTypes: true,
  });
  const entries: Dirent[] = dirents.map((dirent) => {
    return {
      name: dirent.name,
      type: dirent.isDirectory() ? "directory" : "file",
    };
  });
  return {
    entries,
  };
};
