import { CircularProgress, Dialog, Typography } from "@material-ui/core";
import React, { useEffect } from "react";
import endSchedule from "../../store/Action/schedule/endSchedule";
import useStore from "../../store/useStore";
import ScheduleVideo from "../ScheduleVideo";
import errorBackground from "../../../public/image/errorBackground.png";
import errorVideo from "../../../public/video/error.webm";

export default function EatMealWindow() {
  const context = useStore();
  const [state, update] = context;
  const { eatMealWindow } = state.ui;

  useEffect(() => {
    if (!eatMealWindow) {
      return;
    }
    const timeoutId = setTimeout(() => {
      update((state) => endSchedule(state));
    }, 4000);

    return () => clearTimeout(timeoutId);
  }, [eatMealWindow]);

  return (
    <Dialog fullScreen open={eatMealWindow}>
      <div
        style={{
          backgroundImage: `url(${errorBackground})`,
          backgroundSize: "cover",
          backgroundPosition: "center center",
          width: "100vw",
          height: "100vh",
          display: "flex",
          justifyContent: "center",
          alignContent: "center",
          alignItems: "center",
          flexDirection: "column",
        }}
      >
        <CircularProgress />
        <Typography
          variant="h3"
          style={{
            WebkitTextStroke: "2px #000",
            textShadow: "0 0 4px #000",
            color: "#FFF",
            fontWeight: "bold",
          }}
        >
          식사중...
        </Typography>
        <ScheduleVideo source={errorVideo} />
      </div>
    </Dialog>
  );
}
