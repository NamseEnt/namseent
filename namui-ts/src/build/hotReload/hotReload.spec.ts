import { getSavedState } from "./getSavedState";
import { setHotReload } from "./hotReload";
import { isHotReloaded } from "./isHotReloaded";

test("hotReload should reload by request", async () => {
  const src = "bundle/bundle.js";

  const currentScriptRemove = jest.fn();
  const appendedChildren: { src: string }[] = [];

  const currentScript = {
    src,
    remove: currentScriptRemove,
  };

  ((globalThis as any).document as any) = {
    createElement(type: string) {
      return {};
    },
    body: {
      appendChild(element: any) {
        appendedChildren.push(element);
      },
    },
  };

  const buildServerConnection = {
    addEventListener: jest.fn(),
    removeEventListener: jest.fn(),
  };

  const state = {
    a: Symbol(),
  };

  const getState = jest.fn(() => {
    return state;
  });
  const stopEngine = jest.fn();
  await setHotReload({
    buildServerConnection,
    getState,
    stopEngine,
    currentScript: currentScript as any,
  });

  expect(buildServerConnection.addEventListener).toHaveBeenCalledTimes(1);
  expect(buildServerConnection.addEventListener).toHaveBeenCalledWith(
    "reload",
    expect.any(Function),
  );
  buildServerConnection.addEventListener.mock.calls[0][1]();

  expect(stopEngine).toBeCalled();
  expect(currentScriptRemove).toBeCalled();
  expect(appendedChildren.length).toBe(1);
  expect(appendedChildren[0]?.src).toBe(src);
  expect(buildServerConnection.removeEventListener).toHaveBeenCalledTimes(1);

  expect(isHotReloaded()).toBe(true);

  expect(getSavedState()).toEqual(state);
}, 1000);
