import {
  Button,
  Card,
  CardContent,
  CircularProgress,
  Container,
  Dialog,
  Divider,
  Typography,
} from "@material-ui/core";
import React, { useEffect, useRef, useState } from "react";
import endSchedule from "../../store/Action/schedule/endSchedule";
import useStore from "../../store/useStore";
import ScheduleVideo from "../ScheduleVideo";
import StatDisplay from "../StatDisplay";
import errorBackground from "../../../public/image/errorBackground.png";
import errorVideo from "../../../public/video/error.webm";
import BreakSchedule, {
  BreakType,
} from "../../schedule/schedules/BreakSchedule";
import { StatState } from "../../store/State/StatState";
import ParticleOverlay, { ParticleOverlayRef } from "../ParticleOverlay";
import emitTextPoppingEffect from "../EvaluationWindow/GameManager/ParticleManager/emitters/emitTextPoppingEffect";
import Vector2D from "../EvaluationWindow/GameManager/ParticleManager/Vector2D";
import Color from "../EvaluationWindow/GameManager/ColorManager/Color";

function getDescription(type?: BreakType) {
  switch (type) {
    case "cafe": {
      return "카페가기";
    }
    case "game": {
      return "게임하기";
    }
    case "nap": {
      return "낮잠자기";
    }
    case "reading": {
      return "책읽기";
    }
    case "walk": {
      return "산책가기";
    }
    case "watching": {
      return "넷플릭스보기";
    }
    default:
      return "";
  }
}

export default function BreakWindow() {
  const context = useStore();
  const [state, update] = context;
  const { breakWindow } = state.ui;
  const { currentSchedule } = state.schedule;
  const [scheduleDone, setScheduleDone] = useState<boolean>(false);
  const [increment, setIncrement] = useState<Partial<StatState>>({});
  const particleOverlayRef = useRef<ParticleOverlayRef>(null);

  const { subtype, obtains } =
    currentSchedule.type !== "break"
      ? { subtype: undefined, obtains: [] }
      : currentSchedule;

  useEffect(() => {
    if (!breakWindow) {
      return;
    }
    setScheduleDone(false);
    setIncrement({});
    const timeoutIds: NodeJS.Timeout[] = [];
    let accumulatedObtain: BreakSchedule["obtains"][number] = {};
    for (let i = 0; i < obtains.length; i++) {
      timeoutIds.push(
        setTimeout(() => {
          const obtain = obtains[i];
          if (particleOverlayRef.current?.particleManager) {
            const { particleManager, canvasSize } = particleOverlayRef.current;
            const centerX = canvasSize.width / 2;
            const centerY = canvasSize.height / 2;
            const unitSize =
              Math.min(canvasSize.width, canvasSize.height) / 100;
            particleManager
              ? emitTextPoppingEffect({
                  fillColor: new Color({ r: 25, g: 181, b: 254 }),
                  fontSize: 14 * unitSize,
                  lineWidth: 0.5 * unitSize,
                  particleManager: particleOverlayRef.current?.particleManager,
                  position: new Vector2D({ x: centerX, y: centerY }),
                  power: 25,
                  strokeColor: new Color({ r: 255, g: 255, b: 255, a: 1 }),
                  text: "+",
                  unitSize: unitSize,
                })
              : undefined;
          }
          Object.entries(obtain).forEach(([key, value]) => {
            accumulatedObtain[key as keyof typeof obtain] =
              (accumulatedObtain[key as keyof typeof obtain] || 0) + value;
          });
          setIncrement({ ...accumulatedObtain });
        }, i * 500),
      );
    }
    timeoutIds.push(
      setTimeout(() => {
        setScheduleDone(true);
      }, obtains.length * 500),
    );

    return () => timeoutIds.forEach((timeoutId) => clearTimeout(timeoutId));
  }, [breakWindow]);

  return (
    <Dialog fullScreen open={breakWindow} style={{ position: "relative" }}>
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
                {`휴식: ${getDescription(subtype)}`}
              </Typography>
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
