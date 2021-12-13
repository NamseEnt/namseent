import { NotificationData } from "../../notification/type";

export type NotificationState = {
  notifications: NotificationData[];
};

export const initialNotificationState: NotificationState = {
  notifications: [],
};
