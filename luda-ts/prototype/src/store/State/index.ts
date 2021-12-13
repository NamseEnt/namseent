import {
  initialNotificationState,
  NotificationState,
} from "./NotificationState";
import { initialScheduleState, ScheduleState } from "./ScheduleState";
import { initialStatState, StatState } from "./StatState";
import { initialUiState, UiState } from "./UiState";

export type State = {
  ui: UiState;
  schedule: ScheduleState;
  notification: NotificationState;
  stat: StatState;
};

export const initialState: State = {
  ui: initialUiState,
  schedule: initialScheduleState,
  notification: initialNotificationState,
  stat: initialStatState,
};
