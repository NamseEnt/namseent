import { Typography, LinearProgress, Grid } from "@material-ui/core";
import React from "react";
import { StatState } from "../../store/State/StatState";

type StatBarProps = {
  label: keyof StatState;
  value: number;
  increment?: number;
};

const positiveStat: (keyof StatState)[] = [
  "vocal",
  "dance",
  "visual",
  "sense",
  "mentality",
  "health",
  "will",
];

const statMaxLevel: Partial<Record<keyof StatState, number>> = {
  vocal: 20,
  dance: 20,
  visual: 20,
  sense: 20,
  mentality: 20,
  health: 20,
  will: 20,
};

export default function StatBar(props: StatBarProps) {
  const { label, value, increment: increment_ } = props;

  const increment = increment_ || 0;
  const maxLevel = statMaxLevel[label] || 1;
  const maxValue = 100 / maxLevel;
  const currentLevel = Math.floor(value / maxValue);
  const nextLevel = Math.floor((value + increment) / maxValue);
  const { min, max } =
    currentLevel === nextLevel
      ? {
          min: Math.min(value, value + increment) % maxValue,
          max: Math.max(value, value + increment) % maxValue,
        }
      : nextLevel > currentLevel
      ? { min: 0, max: (value + increment) % maxValue }
      : { min: (value + increment) % maxValue, max: maxValue };

  const minPercent = (min / maxValue) * 100;
  const maxPercent = (max / maxValue) * 100;
  const incrementPercent = (increment / maxValue) * 100;

  const color = increment
    ? positiveStat.includes(label)
      ? increment > 0
        ? "primary"
        : "secondary"
      : increment < 0
      ? "primary"
      : "secondary"
    : undefined;

  return (
    <Grid container alignItems="center" spacing={2}>
      <Grid item>
        <Typography variant="body2" color="textSecondary">
          {`${label} ${maxLevel !== 1 ? `Lv.${nextLevel}` : ""}`}
        </Typography>
      </Grid>
      <Grid item xs>
        <LinearProgress
          color={color}
          variant={increment ? "buffer" : "determinate"}
          value={minPercent}
          valueBuffer={maxPercent}
        />
      </Grid>
      <Grid item>
        <Typography variant="body2" color="textSecondary">
          {(increment > 0 ? maxPercent : minPercent).toFixed(0)}%
        </Typography>
      </Grid>
      <Grid item>
        <Typography variant="body2" color={color}>
          {increment
            ? `(${increment > 0 ? "+" : ""}${incrementPercent.toFixed(0)}%)${
                increment > 0 ? "▲" : "▼"
              }`
            : undefined}
        </Typography>
      </Grid>
    </Grid>
  );
}
