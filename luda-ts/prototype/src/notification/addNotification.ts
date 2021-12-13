import { State } from "../store/State";
import { NotificationProp } from "./type";
import { v4 as uuid } from "uuid";

export default function addNotification(
  state: State,
  notification: Omit<NotificationProp, "id">,
) {
  state.notification.notifications.unshift({
    ...notification,
    id: uuid(),
    snackbarEnqueued: false,
  });
}
