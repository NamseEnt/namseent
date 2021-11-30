import { RpcResult } from ".";
import { Dirent } from "./type";

export type ToServerRpcs = {
  ListFiles: {
    input: {
      directoryPath: string;
    };
    output: {
      entries: Dirent[];
    };
  };
  WriteFile: {
    input: {
      destPath: string;
      data: string | ArrayBuffer;
    };
    output: void;
  };
  ReadFile: {
    input: {
      destPath: string;
    };
    output: RpcResult<ArrayBuffer>;
  };
  RenameFile: {
    input: {
      oldPath: string;
      newPath: string;
    };
    output: void;
  };
  RemoveFile: {
    input: {
      destPath: string;
    };
    output: void;
  };
};
