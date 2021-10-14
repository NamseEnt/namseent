import { State } from "../store/State";
import { Day, Time, Week } from "./type";

export default function getScheduleAt(
  state: State,
  week: Week,
  day: Day,
  time: Time,
  defaultOnly?: boolean,
) {
  const {
    reservedSchedule: reservedSchedules,
    defaultSchedule: defaultSchedules,
  } = state.schedule;
  const reservedSchedule = reservedSchedules[week]?.[day]?.[time];
  const defaultSchedule = defaultSchedules[Week.First]?.[day]?.[time];

  return defaultOnly ? defaultSchedule : reservedSchedule || defaultSchedule;
}
