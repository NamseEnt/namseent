export type RpcResult<T> =
  | (T extends void
      ? {
          isSuccessful: true;
        }
      : {
          isSuccessful: true;
          result: T;
        })
  | {
      isSuccessful: false;
      error: string;
    };

export type Dirent = {
  name: string;
  type: "file" | "directory";
};
