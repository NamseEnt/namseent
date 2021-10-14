import { State } from "../../store/State";
import BaseSchedule from "./BaseSchedule";
import error from "../../../public/image/error.png";

type ManageYoutubeScheduleProps = {
  name: string;
  duration?: number;
  editable?: boolean;
};

export default class ManageYoutubeSchedule extends BaseSchedule {
  readonly type = "manageYoutube";
  name: string;
  duration: number;
  editable: boolean;
  thumbnail = error;

  constructor(props: ManageYoutubeScheduleProps) {
    super();
    this.name = props.name;
    this.duration = props.duration ?? 8;
    this.editable = props.editable ?? false;
  }

  startSchedule(state: State): void {
    state.ui.manageYoutubeWindow = true;
  }

  endSchedule(state: State): void {
    state.ui.manageYoutubeWindow = false;
  }

  clone() {
    const newInstance = new ManageYoutubeSchedule({
      name: this.name,
    });
    newInstance.duration = this.duration;
    newInstance.editable = this.editable;
    return newInstance;
  }
}
