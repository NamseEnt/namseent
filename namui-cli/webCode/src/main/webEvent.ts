type WebEvent =
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
      };

const queue: WebEvent[] = [];

export async function waitWebEvent(): Promise<WebEvent> {
    while (queue.length === 0) {
        await new Promise((resolve) => setTimeout(resolve, 0));
    }
    return queue.shift()!;
}

document.addEventListener("mousedown", (e) => {
    e.preventDefault();
    queue.push({
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
    queue.push({
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
    queue.push({
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
    queue.push({
        Wheel: {
            x: e.clientX,
            y: e.clientY,
            deltaX: e.deltaX,
            deltaY: e.deltaY,
        },
    });
});

window.addEventListener("hashchange", (e) => {
    queue.push({
        HashChange: {
            newURL: e.newURL,
            oldURL: e.oldURL,
        },
    });
});

document.addEventListener("dragover", (e) => {
    e.preventDefault();
    queue.push({
        DragOver: {
            x: e.clientX,
            y: e.clientY,
        },
    });
});

// document.addEventListener("drop", (e) => {
//     e.preventDefault();
//     queue.push({
//         Drop: {
//             dataTransfer: e.dataTransfer,
//             x: e.clientX,
//             y: e.clientY,
//         },
//     });
// });

document.addEventListener("selectionchange", (_e) => {
    queue.push("SelectionChange");
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
    queue.push({
        KeyDown: {
            code: e.code,
        },
    });
});

document.addEventListener("keyup", (e) => {
    queue.push({
        KeyUp: {
            code: e.code,
        },
    });
});

document.addEventListener("blur", (_e) => {
    queue.push("Blur");
});

document.addEventListener("visibilitychange", (_e) => {
    queue.push("VisibilityChange");
});

window.addEventListener("blur", (_e) => {
    queue.push("Blur");
});

window.addEventListener("resize", (_e) => {
    queue.push({
        Resize: {
            width: window.innerWidth,
            height: window.innerHeight,
        },
    });
});
