import addNotification from "../../notification/addNotification";
import { NotificationProp } from "../../notification/type";
import endSchedule from "../../store/Action/schedule/endSchedule";
import { State } from "../../store/State";
import BaseInterimSchedule from "./BaseInterimSchedule";

type NotificationInterimScheduleProps = {
  notificationProp: NotificationProp;
};

export default class NotificationInterimSchedule extends BaseInterimSchedule {
  readonly type = "notification";
  notificationProp: NotificationProp;

  constructor(props: NotificationInterimScheduleProps) {
    super();
    this.notificationProp = props.notificationProp;
  }

  startSchedule(state: State): void {
    addNotification(state, this.notificationProp);
    endSchedule(state);
  }

  endSchedule(state: State): void {}
}
