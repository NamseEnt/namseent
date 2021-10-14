import {
  Button,
  Dialog,
  DialogContent,
  DialogTitle,
  Grid,
  Typography,
} from "@material-ui/core";
import React, { useState } from "react";
import useStore from "../../store/useStore";
import DefaultScheduleWindow from "./DefaultScheduleWindow";
import TimeTable from "./TimeTable";

export default function ScheduleWindow() {
  const [state, update] = useStore();
  const { scheduleWindow } = state.ui;
  const { week, day, time } = state.schedule;
  const [defaultScheduleWindowOpen, setDefaultScheduleWindowOpen] =
    useState<boolean>(false);
  return (
    <Dialog fullScreen open={scheduleWindow} style={{ zIndex: 1303 }}>
      <Grid container>
        <Grid item>
          <DialogTitle>Schedule</DialogTitle>
        </Grid>
        <Grid item xs>
          <DialogTitle>
            <Button
              variant="contained"
              onClick={() => setDefaultScheduleWindowOpen(true)}
            >
              set default schedule
            </Button>
          </DialogTitle>
        </Grid>
        <Grid item>
          <Button
            onClick={() => update((state) => (state.ui.scheduleWindow = false))}
          >
            <Typography variant="h4">X</Typography>
          </Button>
        </Grid>
      </Grid>
      <DialogContent>
        <TimeTable week={week} day={day} time={time} />
      </DialogContent>
      <DefaultScheduleWindow
        open={defaultScheduleWindowOpen}
        onClose={() => setDefaultScheduleWindowOpen(false)}
      />
    </Dialog>
  );
}
