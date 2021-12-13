import { State } from "../../store/State";
import BaseInterimSchedule from "./BaseInterimSchedule";

export default class DawnInterimSchedule extends BaseInterimSchedule {
  readonly type = "dawn";

  constructor() {
    super();
  }

  startSchedule(state: State): void {
    state.ui.dawnWindow = true;
  }

  endSchedule(state: State): void {
    state.ui.dawnWindow = false;
  }
}
