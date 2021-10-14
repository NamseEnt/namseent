import { Button, Dialog, DialogContent, Typography } from "@material-ui/core";
import React from "react";
import endSchedule from "../../store/Action/schedule/endSchedule";
import useStore from "../../store/useStore";

export default function ManageYoutubeWindow() {
  const context = useStore();
  const [state, update] = context;
  const { manageYoutubeWindow } = state.ui;
  return (
    <Dialog fullScreen open={manageYoutubeWindow}>
      <DialogContent>
        <Typography>유튜브관리</Typography>
        <Button onClick={() => update((state) => endSchedule(state))}>
          유튜브관리 끝
        </Button>
      </DialogContent>
    </Dialog>
  );
}
