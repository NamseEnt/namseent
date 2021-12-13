import { State } from "../../store/State";
import BaseSchedule from "./BaseSchedule";
import error from "../../../public/image/error.png";

type EatMealScheduleProps = {
  name: string;
  duration?: number;
  editable?: boolean;
};

export default class EatMealSchedule extends BaseSchedule {
  readonly type = "eatMeal";
  name: string;
  duration: number;
  editable: boolean;
  thumbnail = error;

  constructor(props: EatMealScheduleProps) {
    super();
    this.name = props.name;
    this.duration = props.duration ?? 1;
    this.editable = props.editable ?? false;
  }

  startSchedule(state: State): void {
    state.ui.eatMealWindow = true;
  }

  endSchedule(state: State): void {
    state.ui.eatMealWindow = false;
  }

  clone() {
    const newInstance = new EatMealSchedule({
      name: this.name,
    });
    newInstance.duration = this.duration;
    newInstance.editable = this.editable;
    return newInstance;
  }
}
