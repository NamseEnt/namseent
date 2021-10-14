import React, { useEffect } from "react";
import { Button, Grid } from "@material-ui/core";
import useStore from "../store/useStore";
import CommunicationWindow from "./CommunicationWindow";
import EvaluationWindow from "./EvaluationWindow";
import ScheduleWindow from "./ScheduleWindow";
import MenuWindow from "./MenuWindow";
import getScheduleAt from "../schedule/getScheduleAt";
import BreakWindow from "./BreakWindow";
import EatMealWindow from "./EatMealWindow";
import ManageYoutubeWindow from "./ManageYoutubeWindow";
import MinijobWindow from "./MinijobWindow";
import TrainingWindow from "./TrainingWindow";
import { Menu } from "@material-ui/icons";
import setScheduleAt from "../schedule/setScheduleAt";
import getScheduleBefore from "../schedule/getScheduleBefore";
import getScheduleAfter from "../schedule/getScheduleAfter";
import addTime from "../store/Action/schedule/addTime";
import NotificationWindow from "./NotificationWindow";
import BottomBar from "./BottomBar";
import DawnWindow from "./DawnWindow";

export default function App() {
  const context = useStore();
  const [state, update] = context;
  const { week, day, time, inSchedule, stage, lastSchedule } = state.schedule;

  useEffect(() => {
    console.log(stage, inSchedule, lastSchedule);
    if (inSchedule) {
      return;
    }

    switch (stage) {
      case "beforeSchedule": {
        const schedule = getScheduleBefore(state, week, day, time);
        if (!schedule) {
          return update((state) => {
            state.schedule.stage = "schedule";
          });
        }
        return update((state) => {
          state.schedule.currentSchedule = schedule;
          state.schedule.inSchedule = true;
          schedule.startSchedule(state);
        });
      }

      case "schedule": {
        const schedule = getScheduleAt(state, week, day, time);
        if (!schedule) {
          throw new Error("No schedule found");
        }
        return update((state) => {
          state.schedule.currentSchedule = schedule;
          state.schedule.inSchedule = true;
          schedule.startSchedule(state);
          const newSchedule = schedule.clone();
          newSchedule.editable = false;
          setScheduleAt(
            state.schedule.reservedSchedule,
            week,
            day,
            time,
            newSchedule,
          );
        });
      }

      case "afterSchedule": {
        const schedule = getScheduleAfter(state, week, day, time);
        if (!schedule) {
          const mainSchedule = getScheduleAt(state, week, day, time);
          if (!mainSchedule) {
            throw new Error("No schedule found");
          }
          return update((state) => {
            addTime(state, mainSchedule.duration);
            state.schedule.stage = "beforeSchedule";
          });
        }
        return update((state) => {
          state.schedule.currentSchedule = schedule;
          state.schedule.inSchedule = true;
          schedule.startSchedule(state);
        });
      }
    }
  }, [stage, inSchedule, lastSchedule]);

  return (
    <Grid container spacing={2}>
      <BreakWindow />
      <CommunicationWindow />
      <EatMealWindow />
      <EvaluationWindow />
      <ManageYoutubeWindow />
      <MinijobWindow />
      <TrainingWindow />
      <DawnWindow />
      <ScheduleWindow />
      <MenuWindow />
      <BottomBar />
    </Grid>
  );
}
