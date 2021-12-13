import { OptionsObject } from "notistack";

export type NotificationProp = {
  title: string;
  content: string;
  snackbar?: boolean;
  options?: OptionsObject;
};

export type NotificationData = NotificationProp & {
  id: string;
  snackbarEnqueued: boolean;
};
