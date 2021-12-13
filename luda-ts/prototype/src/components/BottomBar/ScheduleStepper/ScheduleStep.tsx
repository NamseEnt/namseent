import React from "react";

type ScheduleStepProps = {
  src: string;
  active: boolean;
};

export default function ScheduleStep(props: ScheduleStepProps) {
  const { src, active } = props;
  return (
    <img
      src={src}
      style={{
        width: "4rem",
        filter: `${active ? "drop-shadow(0 0 4px white)" : "grayscale(100%)"}`,
        opacity: active ? 1 : 0.5,
      }}
    ></img>
  );
}
