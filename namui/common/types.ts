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

export type DeepReadonly<T> = T extends (infer R)[]
  ? DeepReadonlyArray<R>
  : T extends Function
  ? T
  : T extends Map<infer RKey, infer RValue>
  ? ReadonlyMap<RKey, DeepReadonly<RValue>>
  : T extends object
  ? DeepReadonlyObject<T>
  : T;

interface DeepReadonlyArray<T> extends ReadonlyArray<DeepReadonly<T>> {}

type DeepReadonlyObject<T> = {
  readonly [P in keyof T]: DeepReadonly<T[P]>;
};
