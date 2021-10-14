import { State } from "../store/State";
import { Week, Day, Time } from "./type";

export default function getScheduleBefore(
  state: State,
  week: Week,
  day: Day,
  time: Time,
) {
  const { interimSchedule } = state.schedule;
  return interimSchedule[week]?.[day]?.[time]?.before?.[0];
}
