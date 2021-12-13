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
};
