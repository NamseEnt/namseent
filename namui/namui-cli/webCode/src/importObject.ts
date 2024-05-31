import { envGl } from "./envGl";

export function createImportObject({
  memory: importMemory,
  module,
  nextTid,
  wasiImport,
  malloc,
  free,
  webgl,
  memory,
}: {
  memory: WebAssembly.Memory;
  module: WebAssembly.Module;
  nextTid: SharedArrayBuffer;
  wasiImport: Record<string, any>;
  malloc: (size: number) => number;
  free: (ptr: number) => void;
  webgl?: WebGL2RenderingContext;
}) {
  const glFunctions = envGl({
    malloc,
    webgl,
    memory,
  }) as any;

  for (const key in glFunctions) {
    const original = glFunctions[key];
    glFunctions[key] = (...args: (number | bigint)[]) => {
      console.debug(key, args.map((x) => x.toString(16)).join(","));
      return original(...args);
    };
  }

  return {
    env: {
      memory: importMemory,
      ...glFunctions,
      ...implSetJmp({
        memory,
        malloc,
        free,
      }),
    },
    wasi_snapshot_preview1: wasiImport,
    wasi: {
      "thread-spawn": (startArgPtr: number) => {
        const tid = Atomics.add(new Uint32Array(nextTid), 0, 1);
        self.postMessage({
          tid,
          nextTid,
          importMemory,
          module,
          startArgPtr,
        });

        return tid;
      },
    },
    imports: {},
  };
}

// https://github.com/yamt/wasi-libc/blob/a0c169f4facefc1c0d99b000c756e24ef103c2db/libc-top-half/musl/src/setjmp/wasm32/rt.c
function implSetJmp({
  memory,
  malloc,
  free,
}: {
  memory: WebAssembly.Memory;
  malloc: (size: number) => number;
  free: (ptr: number) => void;
}): {
  saveSetjmp: Function;
  testSetjmp: Function;
  getTempRet0: Function;
} {
  // struct entry {
  //   uint32_t id;
  //   uint32_t label;
  // };
  // static _Thread_local struct state {
  //   uint32_t id;
  //   uint32_t size;
  //   struct arg {
  //           void *env;
  //           int val;
  //   } arg;
  // } g_state;

  const gState = {
    id: 0,
    size: 0,
    arg: {
      env: 0,
      val: 0,
    },
  };

  // /*
  // * table is allocated at the entry of functions which call setjmp.
  // *
  // *   table = malloc(40);
  // *   size = 4;
  // *   *(int *)table = 0;
  // */
  // _Static_assert(sizeof(struct entry) * (4 + 1) <= 40, "entry size");
  // void *
  // saveSetjmp(void *env, uint32_t label, void *table, uint32_t size)
  // {
  //   struct state *state = &g_state;
  //   struct entry *e = table;
  //   uint32_t i;
  //   for (i = 0; i < size; i++) {
  //           if (e[i].id == 0) {
  //                   uint32_t id = ++state->id;
  //                   *(uint32_t *)env = id;
  //                   e[i].id = id;
  //                   e[i].label = label;
  //                   /*
  //                    * note: only the first word is zero-initialized
  //                    * by the caller.
  //                    */
  //                   e[i + 1].id = 0;
  //                   goto done;
  //           }
  //   }
  //   size *= 2;
  //   void *p = realloc(table, sizeof(*e) * (size + 1));
  //   if (p == NULL) {
  //           __builtin_trap();
  //   }
  //   table = p;
  // done:
  //   state->size = size;
  //   return table;
  // }

  function saveSetjmp(env: number, label: number, table: number, size: number) {
    const state = gState;
    const entry = new Uint32Array(memory.buffer, table, size * 2);
    for (let i = 0; i < size; i++) {
      if (entry[i * 2] == 0) {
        const id = ++state.id;
        new Uint32Array(memory.buffer, env, 1)[0] = id;
        entry[i * 2] = id;
        entry[i * 2 + 1] = label;
        entry[(i + 1) * 2] = 0;
        return table;
      }
    }
    size *= 2;
    const p = malloc(size * 2 * 4);
    if (p == 0) {
      throw new Error("realloc failed");
    }
    new Uint32Array(memory.buffer, p, size * 2).set(entry);
    free(table);
    return p;
  }

  // uint32_t
  // testSetjmp(unsigned int id, void *table, uint32_t size)
  // {
  //   struct entry *e = table;
  //   uint32_t i;
  //   for (i = 0; i < size; i++) {
  //           if (e[i].id == id) {
  //                   return e[i].label;
  //           }
  //   }
  //   return 0;
  // }

  function testSetjmp(id: number, table: number, size: number) {
    const entry = new Uint32Array(memory.buffer, table, size * 2);
    for (let i = 0; i < size; i++) {
      if (entry[i * 2] == id) {
        return entry[i * 2 + 1];
      }
    }
    return 0;
  }

  // uint32_t
  // getTempRet0()
  // {
  //   struct state *state = &g_state;
  //   return state->size;
  // }

  function getTempRet0() {
    return gState.size;
  }

  return { saveSetjmp, testSetjmp, getTempRet0 };
}
