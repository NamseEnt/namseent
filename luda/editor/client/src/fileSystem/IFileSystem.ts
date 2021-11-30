import { Dirent, RpcResult } from "luda-editor-common";

export interface IFileSystem {
  list(path: string): Promise<Array<Dirent>>;
  read(path: string): Promise<RpcResult<ArrayBuffer>>;
  write(path: string, content: string | ArrayBuffer): Promise<void>;
  rename(oldPath: string, newPath: string): Promise<void>;
  remove(destPath: string): Promise<void>;
}
