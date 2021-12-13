import {
  Button,
  Card,
  DialogTitle,
  Grid,
  List,
  Popover,
  Typography,
} from "@material-ui/core";
import { useSnackbar } from "notistack";
import React, { useEffect } from "react";
import useStore from "../../store/useStore";
import NotificationItem from "./NotificationItem";

type NotificationWindowProps = {
  anchorEl: HTMLElement | null;
};

export default function NotificationWindow(props: NotificationWindowProps) {
  const { anchorEl } = props;
  const [state, update] = useStore();
  const { enqueueSnackbar } = useSnackbar();

  const { notificationWindow } = state.ui;
  const { notifications } = state.notification;

  const closeWindow = () =>
    update((state) => (state.ui.notificationWindow = false));

  useEffect(() => {
    update((state) => {
      if (
        state.notification.notifications.some(
          (notificationData) =>
            notificationData.snackbar && !notificationData.snackbarEnqueued,
        )
      ) {
        state.notification.notifications = state.notification.notifications.map(
          (notificationData) => {
            if (
              notificationData.snackbar &&
              !notificationData.snackbarEnqueued
            ) {
              notificationData.snackbarEnqueued = true;
              enqueueSnackbar(notificationData.title, notificationData.options);
            }
            return notificationData;
          },
        );
      }
    });
  }, [notifications]);

  return (
    <Popover
      open={notificationWindow}
      onClose={closeWindow}
      anchorEl={anchorEl}
      anchorOrigin={{
        vertical: "top",
        horizontal: "right",
      }}
      transformOrigin={{
        vertical: "bottom",
        horizontal: "right",
      }}
    >
      <Card
        style={{
          width: "32rem",
          height: "24rem",
          display: "flex",
          flexDirection: "column",
        }}
      >
        <Grid container>
          <Grid item xs>
            <DialogTitle>Notification</DialogTitle>
            {/* <Typography variant="h5">Notification</Typography> */}
          </Grid>
          <Grid item>
            <Button onClick={closeWindow}>
              <Typography variant="h4">X</Typography>
            </Button>
          </Grid>
          <Grid item xs={12}></Grid>
        </Grid>
        <List
          style={{
            margin: "0px 1rem 1rem 1rem",
            boxSizing: "border-box",
            overflow: "auto",
          }}
        >
          {notifications.map((notificationData) => (
            <NotificationItem
              key={`notification-${notificationData.id}`}
              notificationData={notificationData}
            />
          ))}
        </List>
      </Card>
    </Popover>
  );
}
