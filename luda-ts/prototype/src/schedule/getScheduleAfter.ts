import { State } from "../store/State";
import { Day, Time, Week } from "./type";

export default function getScheduleAfter(
  state: State,
  week: Week,
  day: Day,
  time: Time,
) {
  const { interimSchedule } = state.schedule;
  return interimSchedule[week]?.[day]?.[time]?.after?.[0];
}
