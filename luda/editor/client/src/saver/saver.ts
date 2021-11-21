import fileSystem from "../fileSystem/fileSystem";
import { ISaver } from "./ISaver";

class Saver implements ISaver {
  private lastValueString?: string;
  isSaving: boolean = false;
  isUpToDate: boolean = false;

  async autoSave(key: string, value: Object): Promise<void> {
    if (this.isSaving) {
      return;
    }

    const stringifiedValue = this.stringify(value);

    const isFirstTime = this.lastValueString === undefined;
    if (isFirstTime) {
      this.lastValueString = stringifiedValue;
      this.isUpToDate = true;
      return;
    }

    const isValueChanged = this.lastValueString !== stringifiedValue;
    if (!isValueChanged) {
      return;
    }
    this.isUpToDate = false;

    try {
      this.isSaving = true;
      await fileSystem.write(key, stringifiedValue);
      this.lastValueString = stringifiedValue;
      this.isUpToDate = true;
    } catch (error) {
      console.error(error);
    } finally {
      this.isSaving = false;
    }
  }

  stringify(value: any): string {
    return JSON.stringify(value, null, 2);
  }
}

export const saver: ISaver = new Saver();
