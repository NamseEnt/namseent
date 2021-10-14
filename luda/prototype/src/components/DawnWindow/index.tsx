import { Dialog } from "@material-ui/core";
import React, { useEffect } from "react";
import useStore from "../../store/useStore";
import dawnVideo from "../../../public/video/dawn.webm";
import endSchedule from "../../store/Action/schedule/endSchedule";

export default function DawnWindow() {
  const context = useStore();
  const [state, update] = context;
  const { dawnWindow } = state.ui;

  useEffect(() => {
    if (!dawnWindow) {
      return;
    }
    const timeoutId = setTimeout(() => {
      update((state) => endSchedule(state));
    }, 4000);

    return () => clearTimeout(timeoutId);
  }, [dawnWindow]);

  return (
    <Dialog fullScreen open={dawnWindow}>
      <video
        src={dawnVideo}
        autoPlay
        muted
        style={{
          position: "fixed",
          right: 0,
          bottom: 0,
          minWidth: "100%",
          minHeight: "100%",
        }}
      ></video>
    </Dialog>
  );
}
