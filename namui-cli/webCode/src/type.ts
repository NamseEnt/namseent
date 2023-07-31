export type AsyncMessageFromMain = {
    type: "init";
    workerToMainBufferSab: SharedArrayBuffer;
    mainToWorkerBufferSab: SharedArrayBuffer;
    windowWidth: number;
    windowHeight: number;
};

export type AsyncMessageFromWorker =
    | {
          type: "imageBitmap";
          imageBitmap: ImageBitmap;
      }
    | {
          type: "executeAsyncFunction";
          argsNames: string[];
          args: any[];
          code: string;
          id: number;
      };

export type TypedArray =
    | Int8Array
    | Uint8Array
    | Uint8ClampedArray
    | Int16Array
    | Uint16Array
    | Int32Array
    | Uint32Array
    | Float32Array
    | Float64Array;
