import { Container } from "@material-ui/core";
import React from "react";
import { StatState } from "../../store/State/StatState";
import useStore from "../../store/useStore";
import StatBar from "./StatBar";

type StatDisplayProps = {
  increment?: Partial<StatState>;
  showChangedOnly?: boolean;
};

export default function StatDisplay(props: StatDisplayProps) {
  const { increment, showChangedOnly } = props;
  const [state, update] = useStore();
  const { stat } = state;
  return (
    <Container disableGutters>
      {Object.entries(stat).map(([key_, value]) => {
        const key = key_ as keyof typeof stat;
        const statIncrement = (increment || {})[key];
        return showChangedOnly && !statIncrement ? undefined : (
          <StatBar
            label={key}
            value={
              key == "stress"
                ? (value / stat.mentality) * 100
                : key === "tiredness"
                ? (value / stat.health) * 100
                : value
            }
            increment={
              statIncrement
                ? key == "stress"
                  ? (statIncrement / stat.mentality) * 100
                  : key === "tiredness"
                  ? (statIncrement / stat.health) * 100
                  : statIncrement
                : undefined
            }
            key={`stat-${key}-${value}-${statIncrement}`}
          />
        );
      })}
    </Container>
  );
}
