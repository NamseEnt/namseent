import { Box, Button, ButtonGroup } from "@material-ui/core";
import { Menu, Notifications } from "@material-ui/icons";
import React, { useRef } from "react";
import useStore from "../../store/useStore";
import NotificationWindow from "../NotificationWindow";
import ScheduleStepper from "./ScheduleStepper";

export default function BottomBar() {
  const [state, update] = useStore();
  const notificationWindowAnchorEl = useRef<HTMLButtonElement>(null);

  return (
    <Box
      style={{
        position: "fixed",
        right: 0,
        bottom: 0,
        zIndex: 1301,
        flexDirection: "row",
        display: "flex",
        alignItems: "end",
      }}
    >
      <ScheduleStepper />
      <ButtonGroup variant="contained" size="small">
        <Button
          onClick={() =>
            update(
              (state) =>
                (state.ui.notificationWindow = !state.ui.notificationWindow),
            )
          }
          ref={notificationWindowAnchorEl}
        >
          <Notifications />
        </Button>
        <Button onClick={() => update((state) => (state.ui.menuWindow = true))}>
          <Menu />
        </Button>
        <NotificationWindow anchorEl={notificationWindowAnchorEl.current} />
      </ButtonGroup>
    </Box>
  );
}
