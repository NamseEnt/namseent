import { ToServerRpcHandler } from "luda-editor-common";
import { onListFiles } from "./onListFiles";
import { onWriteFile } from "./onWriteFile";

export type ToServerRpcHandlerContext = {};

export const toServerRpcHandler: ToServerRpcHandler<ToServerRpcHandlerContext> =
  {
    onListFiles,
    onWriteFile,
  };
