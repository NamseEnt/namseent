import { toServerRpcHandler } from "./toServerRpcHandler";
import fs from "fs/promises";
import path from "path";
import { resourcesRoot } from "./resourcesRoot";

export const onReadFile: typeof toServerRpcHandler["onReadFile"] = async (
  context,
  { destPath },
) => {
  const destPathAbsolute = path.join(resourcesRoot, destPath);
  try {
    const file = await fs.readFile(destPathAbsolute);
    return {
      isSuccessful: true,
      result: file,
    };
  } catch (error: any) {
    const errorCode = (error.code || error.toString()) as string;
    return {
      isSuccessful: false,
      error: errorCode,
    };
  }
  return {
    isSuccessful: false,
    error: "Uncaught Error",
  };
};
