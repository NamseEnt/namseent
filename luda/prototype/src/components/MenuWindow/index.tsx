import {
  Button,
  Dialog,
  DialogContent,
  DialogTitle,
  Grid,
  Typography,
} from "@material-ui/core";
import React from "react";
import useStore from "../../store/useStore";

export default function MenuWindow() {
  const [state, update] = useStore();
  const { menuWindow } = state.ui;
  return (
    <Dialog
      open={menuWindow}
      style={{
        zIndex: 1302,
      }}
    >
      <Grid container>
        <Grid item xs>
          <DialogTitle>Menu</DialogTitle>
        </Grid>
        <Grid item>
          <Button
            onClick={() => update((state) => (state.ui.menuWindow = false))}
          >
            <Typography variant="h4">X</Typography>
          </Button>
        </Grid>
      </Grid>
      <DialogContent>
        <Button
          variant="contained"
          fullWidth
          onClick={() => update((state) => (state.ui.scheduleWindow = true))}
        >
          Schedule
        </Button>
      </DialogContent>
    </Dialog>
  );
}
