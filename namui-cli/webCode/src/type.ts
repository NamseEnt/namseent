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
