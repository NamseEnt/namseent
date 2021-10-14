import { State } from "../../store/State";
import BaseSchedule from "./BaseSchedule";
import error from "../../../public/image/error.png";

type OffScheduleProps = {
  name: string;
  duration?: number;
  editable?: boolean;
};

export default class OffSchedule extends BaseSchedule {
  readonly type = "off";
  name: string;
  duration: number;
  editable: boolean;
  thumbnail = error;

  constructor(props: OffScheduleProps) {
    super();
    this.name = props.name;
    this.duration = props.duration ?? 1;
    this.editable = props.editable ?? false;
  }

  startSchedule(state: State): void {
    this.endSchedule(state);
  }

  endSchedule(state: State): void {}

  clone() {
    const newInstance = new OffSchedule({
      name: this.name,
    });
    newInstance.duration = this.duration;
    newInstance.editable = this.editable;
    return newInstance;
  }
}
