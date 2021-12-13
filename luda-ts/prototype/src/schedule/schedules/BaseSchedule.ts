import { State } from "../../store/State";

export default abstract class BaseSchedule {
  abstract readonly type: string;
  abstract name: string;
  abstract duration: number;
  abstract editable: boolean;
  abstract startSchedule(state: State): void;
  abstract endSchedule(state: State): void;
  abstract clone(): BaseSchedule;
  abstract thumbnail: string;
}
