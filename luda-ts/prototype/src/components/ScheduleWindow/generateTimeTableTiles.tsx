import React from "react";
import convertDayTimeToTime from "../../schedule/convertDayTimeToTime";
import convertTimeToDayTime from "../../schedule/convertTimeToDayTime";
import getScheduleAt from "../../schedule/getScheduleAt";
import { Day, Time, Week } from "../../schedule/type";
import { State } from "../../store/State";
import TimeTableTile from "./TimeTableTile";

export default function generateTimeTableTiles(
  state: State,
  week: Week,
  day: Day,
  time: Time,
  dayOffset: number,
  clickHandler: (week: Week, day: Day, time: Time) => void,
  defaultOnly?: boolean,
): JSX.Element[] {
  const timeTableTiles: JSX.Element[] = [];
  const totalTimeOffset =
    convertDayTimeToTime(week, day, Time.A) + dayOffset * 8;
  for (let dayIndex = 0; dayIndex < 7; dayIndex++) {
    const columnTotalTime = totalTimeOffset + dayIndex * 8;
    if (columnTotalTime < 0) {
      continue;
    }
    const [_, columnDay] = convertTimeToDayTime(columnTotalTime);
    timeTableTiles.push(
      <TimeTableTile
        key={`${defaultOnly ? "default-" : ""}timeTable-day-${day + dayIndex}`}
        row={1}
        column={dayIndex + 1}
        duration={1}
        title={columnDay}
        content=""
      />,
    );

    let timeIndex = 0;
    while (timeIndex < 8) {
      const tileTotalTime = columnTotalTime + timeIndex;
      const [tileWeek, tileDay, tileTime] = convertTimeToDayTime(tileTotalTime);
      const schedule = getScheduleAt(
        state,
        tileWeek,
        tileDay,
        tileTime,
        defaultOnly,
      );

      if (!schedule) {
        throw new Error("No schedule found");
      }
      let content = "";
      switch (schedule.type) {
        case "break": {
          break;
        }
        case "minijob": {
          content = schedule.subtype;
          break;
        }
        case "training": {
          content = `${schedule.subtype} ${schedule.difficulty + 1}`;
          break;
        }
        default: {
          content = "";
          break;
        }
      }

      timeTableTiles.push(
        <TimeTableTile
          key={`${
            defaultOnly ? "default-" : ""
          }timeTable-tile-${tileWeek}-${tileDay}-${tileTime}`}
          row={timeIndex + 2}
          column={dayIndex + 1}
          duration={schedule.duration}
          title={schedule.name}
          content={content}
          badge={
            !defaultOnly &&
            tileWeek === week &&
            tileDay === day &&
            tileTime === time
              ? "Now"
              : undefined
          }
          disabled={!schedule.editable}
          thumbnail={schedule.thumbnail}
          clickHandler={() => clickHandler(tileWeek, tileDay, tileTime)}
        />,
      );
      timeIndex += schedule.duration;
    }
  }

  return timeTableTiles;
}
