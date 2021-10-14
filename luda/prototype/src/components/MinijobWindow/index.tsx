import {
  Button,
  Card,
  CardContent,
  CircularProgress,
  Container,
  Dialog,
  Divider,
  LinearProgress,
  Typography,
} from "@material-ui/core";
import React, { useEffect, useRef, useState } from "react";
import endSchedule from "../../store/Action/schedule/endSchedule";
import useStore from "../../store/useStore";
import ScheduleVideo from "../ScheduleVideo";
import StatDisplay from "../StatDisplay";
import { MinijobType } from "../../schedule/schedules/MinijobSchedule";
import errorBackground from "../../../public/image/errorBackground.png";
import errorVideo from "../../../public/video/error.webm";
import { StatState } from "../../store/State/StatState";
import Color from "../EvaluationWindow/GameManager/ColorManager/Color";
import emitTextPoppingEffect from "../EvaluationWindow/GameManager/ParticleManager/emitters/emitTextPoppingEffect";
import Vector2D from "../EvaluationWindow/GameManager/ParticleManager/Vector2D";
import ParticleOverlay, { ParticleOverlayRef } from "../ParticleOverlay";

function getDescription(type?: MinijobType) {
  let typeDescription = "";
  switch (type) {
    case "event": {
      typeDescription = "행사";
      break;
    }
    case "fittingModel": {
      typeDescription = "피팅모델";
      break;
    }
    case "mascotSuit": {
      typeDescription = "인형탈";
      break;
    }
    case "weddingSong": {
      typeDescription = "축가";
      break;
    }
    default:
      break;
  }

  return `${typeDescription} 아르바이트`;
}

function getSuccessRank(successRate: number) {
  switch (5 - successRate) {
    case 0: {
      return "S";
    }
    case 1: {
      return "A";
    }
    case 2: {
      return "B";
    }
    case 3: {
      return "C";
    }
    case 4: {
      return "D";
    }
    default: {
      return "F";
    }
  }
}

export default function MinijobWindow() {
  const context = useStore();
  const [state, update] = context;
  const { minijobWindow } = state.ui;
  const { currentSchedule } = state.schedule;
  const [scheduleDone, setScheduleDone] = useState<boolean>(false);
  const [increment, setIncrement] = useState<Partial<StatState>>({});
  const [successRate, setSuccessRate] = useState<number>(0);
  const particleOverlayRef = useRef<ParticleOverlayRef>(null);

  const { subtype, obtains } =
    currentSchedule.type !== "minijob"
      ? { subtype: undefined, obtains: [] }
      : currentSchedule;

  useEffect(() => {
    if (!minijobWindow) {
      return;
    }
    setScheduleDone(false);
    setSuccessRate(0);
    setIncrement({});
    const timeoutIds: NodeJS.Timeout[] = [];
    const accumulatedIncrement: Partial<StatState> = {};
    for (let i = 0; i < obtains.length; i++) {
      timeoutIds.push(
        setTimeout(() => {
          const obtain = obtains[i];
          const { increment, success } = obtain;
          setSuccessRate((prev) => prev + (success ? 1 : 0));
          if (particleOverlayRef.current?.particleManager) {
            const { particleManager, canvasSize } = particleOverlayRef.current;
            const centerX = canvasSize.width / 2;
            const centerY = canvasSize.height / 2;
            const unitSize =
              Math.min(canvasSize.width, canvasSize.height) / 100;
            particleManager
              ? emitTextPoppingEffect({
                  fillColor: success
                    ? new Color({ r: 25, g: 181, b: 254 })
                    : new Color({ r: 240, g: 52, b: 52 }),
                  fontSize: 14 * unitSize,
                  lineWidth: 0.5 * unitSize,
                  particleManager: particleOverlayRef.current?.particleManager,
                  position: new Vector2D({ x: centerX, y: centerY }),
                  power: 25,
                  strokeColor: new Color({ r: 255, g: 255, b: 255, a: 1 }),
                  text: success ? "+" : "-",
                  unitSize: unitSize,
                })
              : undefined;
          }
          Object.entries(obtain.increment).forEach(([key_, value]) => {
            const key = key_ as keyof typeof increment;
            accumulatedIncrement[key] =
              (accumulatedIncrement[key] || 0) + value;
          });
          setIncrement({ ...accumulatedIncrement });
        }, i * 500),
      );
    }
    timeoutIds.push(
      setTimeout(() => {
        setScheduleDone(true);
      }, obtains.length * 500),
    );

    return () => timeoutIds.forEach((timeoutId) => clearTimeout(timeoutId));
  }, [minijobWindow]);

  return (
    <Dialog fullScreen open={minijobWindow} style={{ position: "relative" }}>
      <ParticleOverlay ref={particleOverlayRef} />
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
        <Container maxWidth="md">
          <Card>
            <CardContent>
              <Typography
                variant="h3"
                style={{
                  WebkitTextStroke: "2px #000",
                  textShadow: "0 0 4px #000",
                  color: "#FFF",
                  fontWeight: "bold",
                }}
              >
                {getDescription(subtype)}
              </Typography>
            </CardContent>
            <Divider variant="middle" />
            <CardContent>
              <Typography variant="h5">성공도</Typography>
              <Typography variant="h4">
                {getSuccessRank(successRate)}
              </Typography>
              <LinearProgress
                variant="determinate"
                value={(successRate / 6) * 100}
              />
            </CardContent>
            <Divider variant="middle" />
            <CardContent>
              <Typography variant="h5">능력치</Typography>
              <StatDisplay showChangedOnly increment={increment} />
              <Button
                disabled={!scheduleDone}
                fullWidth
                onClick={() =>
                  update((state) => {
                    Object.entries(increment).forEach(
                      ([key, value]) =>
                        (state.stat[key as keyof StatState] += value),
                    );
                    endSchedule(state);
                  })
                }
              >
                {scheduleDone ? "확인" : <CircularProgress />}
              </Button>
            </CardContent>
          </Card>
        </Container>
        <ScheduleVideo source={errorVideo} />
      </div>
    </Dialog>
  );
}
