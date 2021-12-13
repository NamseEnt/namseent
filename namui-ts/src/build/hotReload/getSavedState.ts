import { isHotReloaded } from "./isHotReloaded";

export function getSavedState(): any {
  if (!isHotReloaded()) {
    throw new Error("it is not hot reloaded");
  }
  return globalThis.hotReloadModule!.state;
}
