import { envGl } from "./envGl";
import { EventSystemOnWorker } from "../eventSystem";
import { sendMessageToMainThread } from "../interWorkerProtocol";
import { textInputImports } from "./textInput";
import { Exports } from "../exports";
import { webSocketImports } from "../webSocket";
import { insertJsImports } from "../insertJs";
import { storageImports } from "../storage/imports";
import { bufferPoolImports } from "../bufferPool";
import { newEventSystemImports } from "../newEventSystem";
import { httpFetchImports } from "../httpFetch/httpFetch";
import { audioImports } from "../audio";

export function createImportObject({
    memory,
    module,
    nextTid,
    wasiImport,
    canvas,
    eventBuffer,
    initialWindowWh,
    exports,
    bundleSqlite,
    storageProtocolBuffer,
}: {
    memory: WebAssembly.Memory;
    module: WebAssembly.Module;
    nextTid: SharedArrayBuffer;
    wasiImport: Record<string, any>;
    canvas?: OffscreenCanvas;
    eventBuffer: SharedArrayBuffer;
    initialWindowWh: number;
    exports: () => Exports;
    bundleSqlite: () => ArrayBuffer;
    storageProtocolBuffer: SharedArrayBuffer;
}) {
    const glFunctions = envGl({
        exports,
        canvas,
        memory,
    }) as any;

    const glDebug = false;

    if (glDebug) {
        for (const key in glFunctions) {
            const original = glFunctions[key];
            glFunctions[key] = (...args: (number | bigint)[]) => {
                console.debug(
                    key,
                    args.map((x) => `0x${x.toString(16)}`).join(","),
                );
                return original(...args);
            };
        }
    }

    const wasiDebug = false;

    const wasiSnapshotPreview1 = wasiDebug
        ? Object.entries(wasiImport).reduce((acc, [key, value]) => {
              if (value instanceof Function) {
                  acc[key] = (...args: any[]) => {
                      console.debug(
                          key,
                          args.map((x) => x.toString(16)).join(","),
                      );
                      return value(...args);
                  };
              } else {
                  acc[key] = value;
              }
              return acc;
          }, {} as Record<string, any>)
        : wasiImport;

    let eventSystem: EventSystemOnWorker;

    return {
        env: {
            memory,
            ...glFunctions,
            ...implSetJmp({
                memory,
                exports,
            }),
            ...textInputImports({
                memory,
            }),
            ...webSocketImports({
                memory,
            }),
            ...insertJsImports({
                memory,
            }),
            ...storageImports({
                memory,
                storageProtocolBuffer,
            }),
            ...bufferPoolImports({ memory }),
            ...newEventSystemImports({ memory }),
            ...httpFetchImports({ memory }),
            ...audioImports({ memory }),
            poll_event: (wasmBufferPtr: number): number => {
                if (!eventSystem) {
                    eventSystem = new EventSystemOnWorker(eventBuffer, memory);
                }
                return eventSystem.pollEvent(wasmBufferPtr);
            },
            initial_window_wh: (): number => {
                return initialWindowWh;
            },
            update_canvas_wh: (width: number, height: number) => {
                if (!canvas) {
                    throw new Error("Canvas is not available");
                }
                if (canvas.width !== width) {
                    canvas.width = width;
                }
                if (canvas.height !== height) {
                    canvas.height = height;
                }
                sendMessageToMainThread({
                    type: "update-canvas-wh",
                    width,
                    height,
                });
            },
            take_bitmap: () => {
                if (!canvas) {
                    throw new Error("Canvas is not available");
                }
                const bitmap = canvas.transferToImageBitmap();
                sendMessageToMainThread({
                    type: "bitmap",
                    bitmap,
                });
            },
            _hardware_concurrency: () => {
                return navigator.hardwareConcurrency;
            },
        },
        wasi_snapshot_preview1: wasiSnapshotPreview1,
        wasi: {
            "thread-spawn": (startArgPtr: number) => {
                const tid = Atomics.add(new Uint32Array(nextTid), 0, 1);
                sendMessageToMainThread({
                    type: "thread-spawn",
                    tid,
                    nextTid,
                    wasmMemory: memory,
                    module,
                    startArgPtr,
                    eventBuffer,
                    initialWindowWh,
                    bundleSqlite: bundleSqlite(),
                });

                return tid;
            },
        },
        imports: {},
    };
}

// https://github.com/aheejin/emscripten/blob/878a2f1306e25cce0c1627ef5c06e9f60d85df80/system/lib/compiler-rt/emscripten_setjmp.c
function implSetJmp({
    memory,
    exports,
}: {
    memory: WebAssembly.Memory;
    exports: () => Exports;
}): {
    saveSetjmp: Function;
    testSetjmp: Function;
    getTempRet0: Function;
    setTempRet0: Function;
} {
    // // 0 - Nothing thrown
    // // 1 - Exception thrown
    // // Other values - jmpbuf pointer in the case that longjmp was thrown
    // static uintptr_t setjmpId = 0;
    let setjmpId = 0;
    let tempRet0 = 0;

    function getTempRet0() {
        return tempRet0;
    }

    function setTempRet0() {
        return tempRet0;
    }

    // typedef struct TableEntry {
    //     uintptr_t id;
    //     uint32_t label;
    //   } TableEntry;

    // TableEntry* saveSetjmp(uintptr_t* env, uint32_t label, TableEntry* table, uint32_t size) {
    //     // Not particularly fast: slow table lookup of setjmpId to label. But setjmp
    //     // prevents relooping anyhow, so slowness is to be expected. And typical case
    //     // is 1 setjmp per invocation, or less.
    //     uint32_t i = 0;
    //     setjmpId++;
    //     *env = setjmpId;
    //     while (i < size) {
    //       if (table[i].id == 0) {
    //         table[i].id = setjmpId;
    //         table[i].label = label;
    //         // prepare next slot
    //         table[i + 1].id = 0;
    //         setTempRet0(size);
    //         return table;
    //       }
    //       i++;
    //     }
    //     // grow the table
    //     size *= 2;
    //     table = (TableEntry*)realloc(table, sizeof(TableEntry) * (size +1));
    //     table = saveSetjmp(env, label, table, size);
    //     setTempRet0(size); // FIXME: unneeded?
    //     return table;
    //   }

    function saveSetjmp(
        env: number,
        label: number,
        table: number,
        size: number,
    ) {
        setjmpId++;

        const envBuffer = new Uint32Array(memory.buffer, env, 1);
        envBuffer[0] = setjmpId;

        const tableBuffer = new Uint32Array(memory.buffer, table, size * 2);

        let i = 0;
        while (i < size) {
            const id = tableBuffer[i * 2];
            if (id === 0) {
                tableBuffer[i * 2] = setjmpId;
                tableBuffer[i * 2 + 1] = label;
                // prepare next slot
                tableBuffer[(i + 1) * 2] = 0;
                tempRet0 = size;
                return table;
            }
            i++;
        }

        size *= 2;
        exports().free(table);
        table = exports().malloc((size + 1) * 8);
        table = saveSetjmp(env, label, table, size);
        tempRet0 = size; // FIXME: unneeded?
        return table;
    }

    // uint32_t testSetjmp(uintptr_t id, TableEntry* table, uint32_t size) {
    //     uint32_t i = 0;
    //     while (i < size) {
    //       uintptr_t curr = table[i].id;
    //       if (curr == 0) break;
    //       if (curr == id) {
    //         return table[i].label;
    //       }
    //       i++;
    //     }
    //     return 0;
    //   }

    function testSetjmp(id: number, table: number, size: number) {
        const tableBuffer = new Uint32Array(memory.buffer, table, size * 2);

        let i = 0;
        while (i < size) {
            const curr = tableBuffer[i * 2];
            if (curr === 0) break;
            if (curr === id) {
                return tableBuffer[i * 2 + 1];
            }
            i++;
        }
        return 0;
    }

    return { saveSetjmp, testSetjmp, getTempRet0, setTempRet0 };
}
