import {
  Button,
  Dialog,
  DialogContent,
  DialogTitle,
  Grid,
  Typography,
} from "@material-ui/core";
import React from "react";
import { Day, Time, Week } from "../../schedule/type";
import TimeTable from "./TimeTable";

type DefaultScheduleWindowProps = {
  open: boolean;
  onClose: () => void;
};

export default function DefaultScheduleWindow(
  props: DefaultScheduleWindowProps,
) {
  const { open, onClose } = props;

  return (
    <Dialog fullScreen open={open} style={{ zIndex: 1304 }}>
      <Grid container>
        <Grid item xs>
          <DialogTitle>Default Schedule</DialogTitle>
        </Grid>
        <Grid item>
          <Button onClick={() => onClose()}>
            <Typography variant="h4">X</Typography>
          </Button>
        </Grid>
      </Grid>
      <DialogContent>
        <TimeTable
          week={Week.First}
          day={Day.Monday}
          time={Time.A}
          defaultOnly={true}
        />
      </DialogContent>
    </Dialog>
  );
}
