import fileSystem from "../fileSystem/fileSystem";
import { ISaver } from "./ISaver";

class Saver implements ISaver {
  private lastValueString?: string;
  private isSaving_: boolean = false;
  private isUpToDate_: boolean = false;

  async autoSave(key: string, value: Object): Promise<void> {
    if (this.isSaving_) {
      return;
    }

    const stringifiedValue = this.stringify(value);

    const isFirstTime = this.lastValueString === undefined;
    if (isFirstTime) {
      this.lastValueString = stringifiedValue;
      this.isUpToDate_ = true;
      return;
    }

    const isValueChanged = this.lastValueString !== stringifiedValue;
    if (!isValueChanged) {
      return;
    }
    this.isSaving_ = true;
    this.isUpToDate_ = false;
    try {
      await fileSystem.write(key, stringifiedValue);
      this.lastValueString = stringifiedValue;
      this.isUpToDate_ = true;
    } catch {}
    this.isSaving_ = false;
  }

  get isSaving() {
    return this.isSaving_;
  }
  get isUpToDate() {
    return this.isUpToDate_;
  }

  stringify(value: any): string {
    return JSON.stringify(value, null, 2);
  }
}

export const saver = new Saver();
