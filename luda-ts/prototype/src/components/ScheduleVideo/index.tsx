import React from "react";

type ScheduleVideoProps = {
  source?: string;
};

export default function ScheduleVideo(props: ScheduleVideoProps) {
  const { source } = props;

  return (
    <video
      src={source}
      autoPlay
      muted
      loop
      style={{
        width: "40vmin",
      }}
    ></video>
  );
}
