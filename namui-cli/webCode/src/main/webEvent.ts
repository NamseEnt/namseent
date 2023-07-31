export type WebEvent =
    | {
          MouseDown: {
              x: number;
              y: number;
              button: number;
              buttons: number;
          };
      }
    | {
          MouseMove: {
              x: number;
              y: number;
              button: number;
              buttons: number;
          };
      }
    | {
          MouseUp: {
              x: number;
              y: number;
              button: number;
              buttons: number;
          };
      }
    | {
          Wheel: {
              x: number;
              y: number;
              deltaX: number;
              deltaY: number;
          };
      }
    | {
          HashChange: {
              newURL: string;
              oldURL: string;
          };
      }
    | {
          DragOver: {
              x: number;
              y: number;
          };
      }
    // | {
    //       Drop: {
    //           dataTransfer: DataTransfer | null;
    //           x: number;
    //           y: number;
    //       };
    //   }
    | "SelectionChange"
    | {
          KeyDown: {
              code: string;
          };
      }
    | {
          KeyUp: {
              code: string;
          };
      }
    | "Blur"
    | "VisibilityChange"
    | {
          Resize: {
              width: number;
              height: number;
          };
      }
    | {
          AsyncFunction: {
              result: any; // this is saved in worker as JsValue.
              id: number;
          };
      };

const queue: WebEvent[] = [];

export async function shiftWebEvent(): Promise<WebEvent | undefined> {
    return queue.shift();
}

export function enqueueWebEvent(event: WebEvent) {
    queue.push(event);
}

document.addEventListener("mousedown", (e) => {
    e.preventDefault();
    enqueueWebEvent({
        MouseDown: {
            x: e.clientX,
            y: e.clientY,
            button: e.button,
            buttons: e.buttons,
        },
    });
});

document.addEventListener("mousemove", (e) => {
    e.preventDefault();
    enqueueWebEvent({
        MouseMove: {
            x: e.clientX,
            y: e.clientY,
            button: e.button,
            buttons: e.buttons,
        },
    });
});

document.addEventListener("mouseup", (e) => {
    e.preventDefault();
    enqueueWebEvent({
        MouseUp: {
            x: e.clientX,
            y: e.clientY,
            button: e.button,
            buttons: e.buttons,
        },
    });
});

document.addEventListener("wheel", (e) => {
    e.preventDefault();
    enqueueWebEvent({
        Wheel: {
            x: e.clientX,
            y: e.clientY,
            deltaX: e.deltaX,
            deltaY: e.deltaY,
        },
    });
});

window.addEventListener("hashchange", (e) => {
    enqueueWebEvent({
        HashChange: {
            newURL: e.newURL,
            oldURL: e.oldURL,
        },
    });
});

document.addEventListener("dragover", (e) => {
    e.preventDefault();
    enqueueWebEvent({
        DragOver: {
            x: e.clientX,
            y: e.clientY,
        },
    });
});

// document.addEventListener("drop", (e) => {
//     e.preventDefault();
//     enqueueWebEvent({
//         Drop: {
//             dataTransfer: e.dataTransfer,
//             x: e.clientX,
//             y: e.clientY,
//         },
//     });
// });

document.addEventListener("selectionchange", (_e) => {
    enqueueWebEvent("SelectionChange");
});

document.addEventListener("keydown", (e) => {
    const isDevToolOpenCalled =
        e.code === "F12" || (e.ctrlKey && e.shiftKey && e.code === "KeyI");
    const isRefreshCalled = e.code === "F5" || (e.ctrlKey && e.code === "KeyR");
    const isJumpToTabCalled =
        e.ctrlKey &&
        (e.code === "Digit1" ||
            e.code === "Digit2" ||
            e.code === "Digit3" ||
            e.code === "Digit4" ||
            e.code === "Digit5" ||
            e.code === "Digit6" ||
            e.code === "Digit7" ||
            e.code === "Digit8" ||
            e.code === "Digit9" ||
            e.code === "Digit0");
    if (!isDevToolOpenCalled && !isRefreshCalled && !isJumpToTabCalled) {
        e.preventDefault();
    }
    enqueueWebEvent({
        KeyDown: {
            code: e.code,
        },
    });
});

document.addEventListener("keyup", (e) => {
    enqueueWebEvent({
        KeyUp: {
            code: e.code,
        },
    });
});

document.addEventListener("blur", (_e) => {
    enqueueWebEvent("Blur");
});

document.addEventListener("visibilitychange", (_e) => {
    enqueueWebEvent("VisibilityChange");
});

window.addEventListener("blur", (_e) => {
    enqueueWebEvent("Blur");
});

window.addEventListener("resize", (_e) => {
    enqueueWebEvent({
        Resize: {
            width: window.innerWidth,
            height: window.innerHeight,
        },
    });
});
