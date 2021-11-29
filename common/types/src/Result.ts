export type Result<T, TError> =
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
      error: TError;
    };
