import { State } from "../../store/State";

export default abstract class BaseInterimSchedule {
  abstract readonly type: string;
  abstract startSchedule(state: State): void;
  abstract endSchedule(state: State): void;
}
