import { Button, ButtonGroup, Grid } from "@material-ui/core";
import { ArrowBack, ArrowForward, FiberManualRecord } from "@material-ui/icons";
import React, { useState } from "react";
import setScheduleAt from "../../schedule/setScheduleAt";
import { Day, Time, Week } from "../../schedule/type";
import useStore from "../../store/useStore";
import generateTimeTableTiles from "./generateTimeTableTiles";
import generateTimeTableTimeHeaders from "./generateTimeTableTimeHeaders";
import ScheduleSelectorWindow from "./ScheduleSelectorWindow";

type TimeTableProps = {
  week: Week;
  day: Day;
  time: Time;
  defaultOnly?: boolean;
};

export default function TimeTable(props: TimeTableProps) {
  const { week, day, time, defaultOnly } = props;
  const [state, update] = useStore();
  const [offsetDay, setOffsetDay] = useState<number>(0);
  const [selectedWeek, setSelectedWeek] = useState<Week>(Week.First);
  const [selectedDay, setSelectedDay] = useState<Day>(Day.Monday);
  const [selectedTime, setSelectedTime] = useState<Time>(Time.A);
  const [selectorOpen, setSelectorOpen] = useState<boolean>(false);

  return (
    <Grid container spacing={2}>
      <Grid item xs={12}>
        {defaultOnly ? undefined : (
          <ButtonGroup fullWidth>
            <Button onClick={() => setOffsetDay((prev) => prev - 1)}>
              <ArrowBack />
            </Button>
            <Button onClick={() => setOffsetDay(0)}>
              <FiberManualRecord />
            </Button>
            <Button onClick={() => setOffsetDay((prev) => prev + 1)}>
              <ArrowForward />
            </Button>
          </ButtonGroup>
        )}
      </Grid>
      <Grid item xs={2}>
        <div
          style={{
            display: "grid",
            gridTemplateColumns: "1fr",
            gridTemplateRows: "repeat(9, 6rem)",
            gap: "1rem",
          }}
        >
          {generateTimeTableTimeHeaders()}
        </div>
      </Grid>
      <Grid item xs={10} style={{ overflow: "auto" }}>
        <div
          style={{
            display: "grid",
            gridTemplateColumns: "repeat(7, 14rem)",
            gridTemplateRows: "repeat(9, 6rem)",
            gap: "1rem",
          }}
        >
          {generateTimeTableTiles(
            state,
            week,
            day,
            time,
            offsetDay,
            (week, day, time) => {
              setSelectedWeek(week);
              setSelectedDay(day);
              setSelectedTime(time);
              setSelectorOpen(true);
            },
            defaultOnly,
          )}
        </div>
      </Grid>
      <ScheduleSelectorWindow
        open={selectorOpen}
        onSelect={(schedule) => {
          defaultOnly
            ? update((state) =>
                setScheduleAt(
                  state.schedule.defaultSchedule,
                  selectedWeek,
                  selectedDay,
                  selectedTime,
                  schedule,
                ),
              )
            : update((state) =>
                setScheduleAt(
                  state.schedule.reservedSchedule,
                  selectedWeek,
                  selectedDay,
                  selectedTime,
                  schedule,
                ),
              );
          setSelectedWeek(week);
          setSelectedDay(day);
          setSelectedTime(time);
          setSelectorOpen(false);
        }}
      />
    </Grid>
  );
}
