import { envGl } from "./envGl";
import { textInputImports } from "./textInput";
import { type DrawerExports, type Exports } from "@/exports";
import { storageImports } from "@/storage/imports";
import { ThreadStartSupplies } from "@/thread/startThread";
import SubThreadWorker from "@/thread/SubThreadWorker?worker";

export function createImportObject({
    supplies,
    wasiImport,
    exports,
    storageProtocolBuffer,
}: {
    supplies: ThreadStartSupplies;
    wasiImport: Record<string, any>;
    exports: () => DrawerExports | Exports;
    storageProtocolBuffer: SharedArrayBuffer;
}) {
    const { memory } = supplies;
    const glFunctions = envGl({
        exports,
        memory,
        canvas: supplies.type === "drawer" ? supplies.canvas : undefined,
    }) as any;

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
            ...storageImports({
                memory,
                storageProtocolBuffer,
            }),
            _initial_window_wh: () => supplies.initialWindowWh,
            _hardware_concurrency: () => navigator.hardwareConcurrency,
            _get_image_count: () => {
                switch (supplies.type) {
                    case "main":
                    case "sub":
                    case "drawer":
                        return supplies.imageCount;
                    case "font-load":
                        throw new Error(`unreachable on ${supplies.type}`);
                }
            },
            _get_image_infos: (ptr: number) => {
                switch (supplies.type) {
                    case "main":
                    case "sub":
                        return new Uint8Array(
                            memory.buffer,
                            ptr,
                            supplies.imageInfoBytes.length,
                        ).set(supplies.imageInfoBytes);
                    case "drawer":
                        return (exports() as DrawerExports)._image_infos(ptr);
                    case "font-load":
                        throw new Error(`unreachable on ${supplies.type}`);
                }
            },
        },
        wasi_snapshot_preview1: wasiSnapshotPreview1,
        wasi: {
            "thread-spawn": (startArgPtr: number) => {
                const tid = Atomics.add(
                    new Uint32Array(supplies.nextTid),
                    0,
                    1,
                );
                const worker = new SubThreadWorker();
                if (
                    supplies.type === "drawer" ||
                    supplies.type === "font-load"
                ) {
                    throw new Error("not implemented");
                }
                const nextSupplies = {
                    ...supplies,
                    type: "sub",
                    startArgPtr,
                    tid,
                } satisfies ThreadStartSupplies;
                worker.postMessage(nextSupplies);

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
    exports: () => {
        free: (ptr: number) => void;
        malloc: (size: number) => number;
    };
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
