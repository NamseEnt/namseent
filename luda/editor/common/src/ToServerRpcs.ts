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
};
