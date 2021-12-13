import { State } from "../../store/State";
import BaseSchedule from "./BaseSchedule";
import error from "../../../public/image/error.png";

export type EvaluationScheduleProps = {
  name: string;
  duration?: number;
  editable?: boolean;
};

export default class EvaluationSchedule extends BaseSchedule {
  readonly type = "evaluation";
  name: string;
  duration: number;
  editable: boolean;
  thumbnail = error;

  constructor(props: EvaluationScheduleProps) {
    super();
    this.name = props.name;
    this.duration = props.duration ?? 1;
    this.editable = props.editable ?? false;
  }

  startSchedule(state: State): void {
    state.ui.evaluationWindow = true;
  }

  endSchedule(state: State): void {
    state.ui.evaluationWindow = false;
  }

  clone() {
    const newInstance = new EvaluationSchedule({
      name: this.name,
    });
    newInstance.duration = this.duration;
    newInstance.editable = this.editable;
    return newInstance;
  }
}
