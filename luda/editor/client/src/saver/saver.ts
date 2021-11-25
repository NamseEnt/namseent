import { wait } from "utils";
import fileSystem from "../fileSystem/fileSystem";
import { AutoSaveState, ISaver } from "./ISaver";

class Saver implements ISaver {
  private readonly saveStates = new Map<string, AutoSaveState>();
  private readonly lastValueStrings = new Map<string, string>();

  /**
   * known issue:
   * 1. If system restarted by deserializing state,
   *    saver will have no memory.
   *    So saver will always overwrite data because saver think it's the first time to save.
   */
  public autoSave(key: string, value: Object): AutoSaveState {
    const isFirstTime = this.lastValueStrings.get(key) === undefined;
    if (isFirstTime) {
      this.lastValueStrings.set(key, this.stringify(value));
      return AutoSaveState.saved;
    }

    const saveState = this.saveStates.get(key);

    if (!saveState) {
      this.startSaving(key, value);
      return AutoSaveState.saving;
    }

    switch (saveState) {
      case AutoSaveState.saved:
        const isUpToDate = this.checkIsUpToDate(key, value);
        if (isUpToDate) {
          return AutoSaveState.saved;
        }
        this.startSaving(key, value);
        return AutoSaveState.saving;
      default:
        return saveState;
    }
  }
  async startSaving(key: string, value: Object): Promise<void> {
    this.saveStates.set(key, AutoSaveState.saving);

    const stringifiedValue = this.stringify(value);

    const maxRetrial = 5;
    const retrialDelay = 1000;
    for (let trial = 0; trial < maxRetrial; trial++) {
      try {
        await fileSystem.write(key, stringifiedValue);
        this.saveStates.set(key, AutoSaveState.saved);
        return;
      } catch (error) {
        this.saveStates.set(key, AutoSaveState.retryingOnError);
        console.error(error);
        await wait(retrialDelay);
      }
    }

    this.saveStates.set(key, AutoSaveState.failToRecoverError);
  }
  checkIsUpToDate(key: string, value: Object): boolean {
    const lastValueString = this.lastValueStrings.get(key);
    if (lastValueString === undefined) {
      return false;
    }
    const valueString = this.stringify(value);
    return lastValueString === valueString;
  }

  stringify(value: any): string {
    return JSON.stringify(value, null, 2);
  }
}

export const saver: ISaver = new Saver();
