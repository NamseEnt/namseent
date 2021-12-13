import React from "react";
import TimeTableTile from "./TimeTableTile";

export default function generateTimeTableTimeHeaders() {
  return [
    ["A", "10:00~11:15"],
    ["B", "11:30~12:45"],
    ["C", "13:00~14:15"],
    ["D", "14:30~15:45"],
    ["E", "16:00~17:15"],
    ["F", "17:30~18:45"],
    ["G", "19:00~20:15"],
    ["H", "20:30~21:45"],
  ].map((content, index) => (
    <TimeTableTile
      key={`timeTable-time-${content[0]}`}
      row={index + 2}
      column={1}
      duration={1}
      title={`${content[0]}`}
      content={content[1]}
    />
  ));
}
