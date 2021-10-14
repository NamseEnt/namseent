import convertDayTimeToTime from "../../../schedule/convertDayTimeToTime";
import convertTimeToDayTime from "../../../schedule/convertTimeToDayTime";
import { State } from "../../State";

export default function addTime(state: State, amount: number) {
  const { week, day, time } = state.schedule;
  const [newWeek, newDay, newTime] = convertTimeToDayTime(
    convertDayTimeToTime(week, day, time) + amount,
  );

  console.log(amount, week, day, time, newWeek, newDay, newTime);
  state.schedule.week = newWeek;
  state.schedule.day = newDay;
  state.schedule.time = newTime;
}
