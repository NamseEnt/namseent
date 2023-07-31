export type MessateFromMain = {
    type: "init";
    workerToMainBufferSab: SharedArrayBuffer;
    mainToWorkerBufferSab: SharedArrayBuffer;
    windowWidth: number;
    windowHeight: number;
};
