import getScheduleAt from "../../../schedule/getScheduleAt";
import { State } from "../../State";

export default function endSchedule(state: State) {
  const { week, day, time, stage, currentSchedule } = state.schedule;

  const schedule = currentSchedule;
  state.schedule.lastSchedule = schedule;
  schedule.endSchedule(state);
  state.schedule.inSchedule = false;
  if (stage === "schedule") {
    state.schedule.stage = "afterSchedule";
  }
  const weekInterimSchedule = (state.schedule.interimSchedule[week] ??= {});
  const dayInterimSchedule = (weekInterimSchedule[day] ??= {});
  const timeInterimSchedule = (dayInterimSchedule[time] ??= {});
  if (stage === "afterSchedule") {
    const mainSchedule = getScheduleAt(state, week, day, time);
    if (!mainSchedule) {
      throw new Error("Schedule not found. Check defaultSchedule");
    }
    (timeInterimSchedule.after ??= []).pop();
    return;
  }
  (timeInterimSchedule.before ??= []).pop();
}
