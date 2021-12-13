export type VisibilityChangeCallback = (visible: boolean) => void;

export interface IScreenManager {
  onVisibilityChange(callback: VisibilityChangeCallback): void;
}
