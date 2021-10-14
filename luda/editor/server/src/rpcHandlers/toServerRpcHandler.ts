import { ToServerRpcHandler } from "luda-editor-common";
import { onListFiles } from "./onListFiles";

export type ToServerRpcHandlerContext = {};

export const toServerRpcHandler: ToServerRpcHandler<ToServerRpcHandlerContext> =
  {
    onListFiles,
  };
