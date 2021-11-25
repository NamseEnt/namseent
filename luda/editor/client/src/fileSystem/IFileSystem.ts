import { Dirent } from "luda-editor-common";

export interface IFileSystem {
  list(path: string): Promise<Array<Dirent>>;
  read(
    path: string,
  ): Promise<
    | { isSuccessful: true; file: ArrayBuffer }
    | { isSuccessful: false; errorCode: string }
  >;
  write(path: string, content: string | ArrayBuffer): Promise<void>;
  rename(oldPath: string, newPath: string): Promise<void>;
  remove(destPath: string): Promise<void>;
}
