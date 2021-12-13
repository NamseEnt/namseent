import { Day, Schedule, SchedulePlan, Time, Week } from "./type";

export default function setScheduleAt(
  schedulePlan: SchedulePlan,
  week: Week,
  day: Day,
  time: Time,
  schedule?: Schedule,
) {
  const weekSchedule = (schedulePlan[week] ??= {});
  const daySchedule = (weekSchedule[day] ??= {});
  daySchedule[time] = schedule;
}
