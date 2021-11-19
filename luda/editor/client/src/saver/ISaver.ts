export interface ISaver {
  /**
   * @param value
   * Saver will check value needs to be saved or not.
   * 1. If value is not changed, saver will not save it.
   * 2. If value is changed, saver will save it.
   * 3. If autoSave is called first time, saver will not save it.
   * 4. If autoSave was called and saving is not done, saver will not save it.
   */
  autoSave(key: string, value: any): Promise<void>;
  get isSaving(): boolean;
  get isUpToDate(): boolean;
}
