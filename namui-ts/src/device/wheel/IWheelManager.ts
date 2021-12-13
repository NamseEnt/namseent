import { WheelEventCallback } from "../../type";

export interface IWheelManager {
  onWheel(callback: WheelEventCallback): void;
}
