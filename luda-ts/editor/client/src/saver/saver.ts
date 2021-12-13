import fileSystem from "../fileSystem/fileSystem";
import { ISaver } from "./ISaver";

class Saver implements ISaver {
  private lastValueString?: string;
  private isSaving: boolean = false;

  async autoSave(key: string, value: Object): Promise<void> {
    if (this.isSaving) {
      return;
    }

    const stringifiedValue = this.stringify(value);

    const isFirstTime = this.lastValueString === undefined;
    if (isFirstTime) {
      this.lastValueString = stringifiedValue;
      return;
    }

    const isValueChanged = this.lastValueString !== stringifiedValue;
    if (!isValueChanged) {
      return;
    }

    this.lastValueString = stringifiedValue;

    this.isSaving = true;
    await fileSystem.write(key, stringifiedValue);
    this.isSaving = false;
  }

  stringify(value: any): string {
    return JSON.stringify(value, null, 2);
  }
}

export const saver = new Saver();
