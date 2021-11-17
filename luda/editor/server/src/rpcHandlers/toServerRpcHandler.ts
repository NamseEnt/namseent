import { ToServerRpcHandler } from "luda-editor-common";
import { onListFiles } from "./onListFiles";
import { onReadFile } from "./onReadFile";
import { onRenameFile } from "./onRenameFile";
import { onWriteFile } from "./onWriteFile";

export type ToServerRpcHandlerContext = {};

export const toServerRpcHandler: ToServerRpcHandler<ToServerRpcHandlerContext> =
  {
    onListFiles,
    onWriteFile,
    onReadFile,
    onRenameFile,
  };
