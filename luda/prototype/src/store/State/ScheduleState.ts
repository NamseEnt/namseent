import OffSchedule from "../../schedule/schedules/OffSchedule";
import {
  Day,
  InterimSchedule,
  InterimSchedulePlan,
  Schedule,
  SchedulePlan,
  Time,
  Week,
} from "../../schedule/type";
import { initialInterimSchedule } from "./initailInterrimSchedule";
import { initialDefaultSchedule } from "./initialDefaultSchedule";
import { initialReservedSchedule } from "./initialReservedSchedule";

export type ScheduleState = {
  week: Week;
  day: Day;
  time: Time;
  stage: "beforeSchedule" | "schedule" | "afterSchedule";
  inSchedule: boolean;
  currentSchedule: Schedule | InterimSchedule;
  lastSchedule: Schedule | InterimSchedule;
  defaultSchedule: SchedulePlan;
  reservedSchedule: SchedulePlan;
  interimSchedule: InterimSchedulePlan;
};

export const initialScheduleState: ScheduleState = {
  week: Week.First,
  day: Day.Monday,
  time: Time.A,
  stage: "beforeSchedule",
  inSchedule: false,
  currentSchedule: new OffSchedule({ name: "init" }),
  lastSchedule: new OffSchedule({ name: "init" }),
  defaultSchedule: initialDefaultSchedule,
  reservedSchedule: initialReservedSchedule,
  interimSchedule: initialInterimSchedule,
};
