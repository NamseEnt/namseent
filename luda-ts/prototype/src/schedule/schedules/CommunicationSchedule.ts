import { State } from "../../store/State";
import BaseSchedule from "./BaseSchedule";
import error from "../../../public/image/error.png";

export type CommunicationScheduleProps = {
  name: string;
  duration?: number;
  editable?: boolean;
};

export default class CommunicationSchedule extends BaseSchedule {
  readonly type = "communication";
  name: string;
  duration: number;
  editable: boolean;
  thumbnail = error;

  constructor(props: CommunicationScheduleProps) {
    super();
    this.name = props.name;
    this.duration = props.duration ?? 1;
    this.editable = props.editable ?? false;
  }

  startSchedule(state: State): void {
    state.ui.communicationWindow = true;
  }

  endSchedule(state: State): void {
    state.ui.communicationWindow = false;
  }

  clone() {
    const newInstance = new CommunicationSchedule({
      name: this.name,
    });
    newInstance.duration = this.duration;
    newInstance.editable = this.editable;
    return newInstance;
  }
}
