import { State } from "../store/State";
import { Week, Day, Time, InterimSchedule } from "./type";

export default function pushInterimScheduleBefore(
  state: State,
  week: Week,
  day: Day,
  time: Time,
  interimSchedule: InterimSchedule,
) {
  const weekInterimSchedule = (state.schedule.interimSchedule[week] ??= {});
  const dayInterimSchedule = (weekInterimSchedule[day] ??= {});
  const timeInterimSchedule = (dayInterimSchedule[time] ??= {});
  const interimSchedules = (timeInterimSchedule.before ??= []);
  interimSchedules.push(interimSchedule);
}
