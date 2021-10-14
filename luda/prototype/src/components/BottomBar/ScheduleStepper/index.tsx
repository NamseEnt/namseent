import { Box } from "@material-ui/core";
import React, { useState, useEffect } from "react";
import convertDayTimeToTime from "../../../schedule/convertDayTimeToTime";
import convertTimeToDayTime from "../../../schedule/convertTimeToDayTime";
import getScheduleAt from "../../../schedule/getScheduleAt";
import { Time } from "../../../schedule/type";
import useStore from "../../../store/useStore";
import ScheduleStep from "./ScheduleStep";

export default function ScheduleStepper() {
  const [state, update] = useStore();
  const { week, day, time } = state.schedule;
  const [steps, setSteps] = useState<JSX.Element[]>([]);

  useEffect(() => {
    const schedules = [];
    const offsetTime = convertDayTimeToTime(week, day, Time.A);
    const currentSchedule = getScheduleAt(state, week, day, time);
    if (!currentSchedule) {
      return;
    }
    for (let i = 0; i < 8; ) {
      const schedule = getScheduleAt(
        state,
        ...convertTimeToDayTime(offsetTime + i),
      );
      if (!schedule) {
        continue;
      }
      i += schedule.duration;
      schedules.push(schedule);
    }
    setSteps(
      schedules.map((schedule, index) => (
        <ScheduleStep
          key={`schedule-step-${index}`}
          src={schedule.thumbnail}
          active={currentSchedule === schedule}
        />
      )),
    );
  }, [week, day, time]);

  return <Box>{steps}</Box>;
}
