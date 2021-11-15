import { toServerRpcHandler } from "./toServerRpcHandler";
import fs from "fs/promises";
import path from "path";
import { resourcesRoot } from "./resourcesRoot";

export const onWriteFile: typeof toServerRpcHandler["onWriteFile"] = async (
  context,
  { data, destPath },
) => {
  const destPathAbsolute = path.join(resourcesRoot, destPath);
  const buffer =
    typeof data === "string" ? Buffer.from(data, "utf8") : Buffer.from(data);
  await fs.writeFile(destPathAbsolute, buffer);

  return;
};
