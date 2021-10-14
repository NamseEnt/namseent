import DawnInterimSchedule from "./interimSchedules/DawnInterimSchedule";
import NotificationInterimSchedule from "./interimSchedules/NotificationInterimSchedule";
import BreakSchedule from "./schedules/BreakSchedule";
import CommunicationSchedule from "./schedules/CommunicationSchedule";
import EatMealSchedule from "./schedules/EatMealSchedule";
import EvaluationSchedule from "./schedules/EvaluationSchedule";
import ManageYoutubeSchedule from "./schedules/ManageYoutubeSchedule";
import MinijobSchedule from "./schedules/MinijobSchedule";
import OffSchedule from "./schedules/OffSchedule";
import TrainingSchedule from "./schedules/TrainingSchedule";

export type SchedulePlan = Partial<{
  [week in Week]: Partial<{
    [day in Day]: Partial<{
      [time in Time]: Schedule;
    }>;
  }>;
}>;

export type InterimSchedulePlan = Partial<{
  [week in Week]: Partial<{
    [day in Day]: Partial<{
      [time in Time]: Partial<{
        before: InterimSchedule[];
        after: InterimSchedule[];
      }>;
    }>;
  }>;
}>;

export type Schedule =
  | BreakSchedule
  | CommunicationSchedule
  | EatMealSchedule
  | EvaluationSchedule
  | ManageYoutubeSchedule
  | MinijobSchedule
  | OffSchedule
  | TrainingSchedule;

export type InterimSchedule =
  | Schedule
  | NotificationInterimSchedule
  | DawnInterimSchedule;

export type ScheduleType = Schedule["type"];

export enum Week {
  First = "First",
  Second = "Second",
  Third = "Third",
  Fourth = "Fourth",
  Fifth = "Fifth",
  Sixth = "Sixth",
  Seventh = "Seventh",
  Eighth = "Eighth",
  Ninth = "Ninth",
  Tenth = "Tenth",
  Eleventh = "Eleventh",
  Twelfth = "Twelfth",
  Thirteenth = "Thirteenth",
}

export enum Day {
  Monday = "Monday",
  Tuesday = "Tuesday",
  Wednesday = "Wednesday",
  Thursday = "Thursday",
  Friday = "Friday",
  Saturday = "Saturday",
  Sunday = "Sunday",
}

export enum Time {
  A = "A",
  B = "B",
  C = "C",
  D = "D",
  E = "E",
  F = "F",
  G = "G",
  H = "H",
}
