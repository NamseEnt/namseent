import React, { useEffect, useRef, useState } from "react";
import endSchedule from "../../store/Action/schedule/endSchedule";
import useStore from "../../store/useStore";
import vocalBeginner from "../../../public/video/training/vocal/vocalBeginner.webm";
import vocalIntermediate from "../../../public/video/training/vocal/vocalIntermediate.webm";
import vocalAdvanced from "../../../public/video/training/vocal/vocalAdvanced.webm";
import danceBeginner from "../../../public/video/training/dance/danceBeginner.webm";
import danceIntermediate from "../../../public/video/training/dance/danceIntermediate.webm";
import danceAdvanced from "../../../public/video/training/dance/danceAdvanced.webm";
import weightBeginner from "../../../public/video/training/weight/weightBeginner.webm";
import weightIntermediate from "../../../public/video/training/weight/weightIntermediate.webm";
import weightAdvanced from "../../../public/video/training/weight/weightAdvanced.webm";
import songWriteBeginner from "../../../public/video/training/songWrite/songWriteBeginner.webm";
import songWriteIntermediate from "../../../public/video/training/songWrite/songWriteIntermediate.webm";
import songWriteAdvanced from "../../../public/video/training/songWrite/songWriteAdvanced.webm";
import vocalBackground from "../../../public/image/training/background/vocalBackground.png";
import danceBackground from "../../../public/image/training/background/danceBackground.png";
import weightBackground from "../../../public/image/training/background/weightBackground.png";
import { TrainingType } from "../../schedule/schedules/TrainingSchedule";
import ScheduleVideo from "../ScheduleVideo";
import StatDisplay from "../StatDisplay";
import {
  Dialog,
  CircularProgress,
  Typography,
  LinearProgress,
  Divider,
  Button,
  Card,
  CardContent,
  Container,
} from "@material-ui/core";
import { StatState } from "../../store/State/StatState";
import Color from "../EvaluationWindow/GameManager/ColorManager/Color";
import emitTextPoppingEffect from "../EvaluationWindow/GameManager/ParticleManager/emitters/emitTextPoppingEffect";
import Vector2D from "../EvaluationWindow/GameManager/ParticleManager/Vector2D";
import ParticleOverlay, { ParticleOverlayRef } from "../ParticleOverlay";

const trainingVideoSourceMap: Record<TrainingType, Record<number, string>> = {
  vocal: {
    0: vocalBeginner,
    1: vocalIntermediate,
    2: vocalAdvanced,
  },
  dance: {
    0: danceBeginner,
    1: danceIntermediate,
    2: danceAdvanced,
  },
  weight: {
    0: weightBeginner,
    1: weightIntermediate,
    2: weightAdvanced,
  },
  songWrite: {
    0: songWriteBeginner,
    1: songWriteIntermediate,
    2: songWriteAdvanced,
  },
};

const trainingBackgroundSourceMap: Record<TrainingType, string> = {
  vocal: vocalBackground,
  dance: danceBackground,
  weight: weightBackground,
  songWrite: vocalBackground,
};

function getDescription(type?: TrainingType, difficulty?: number) {
  let typeDescription = "";
  switch (type) {
    case "vocal": {
      typeDescription = "보컬";
      break;
    }
    case "dance": {
      typeDescription = "댄스";
      break;
    }
    case "weight": {
      typeDescription = "웨이트";
      break;
    }
    case "songWrite": {
      typeDescription = "작곡";
      break;
    }
    default:
      break;
  }

  let difficultyDescription = "";
  switch (difficulty) {
    case 0: {
      difficultyDescription = "초급";
      break;
    }
    case 1: {
      difficultyDescription = "중급";
      break;
    }
    case 2: {
      difficultyDescription = "고급";
      break;
    }
    default:
      break;
  }

  return `${difficultyDescription} ${typeDescription} 트레이닝`;
}

function getSuccessRank(successRate: number) {
  switch (5 - successRate) {
    case 0: {
      return "S";
      break;
    }
    case 1: {
      return "A";
      break;
    }
    case 2: {
      return "B";
      break;
    }
    case 3: {
      return "C";
      break;
    }
    case 4: {
      return "D";
      break;
    }
    default: {
      return "F";
      break;
    }
  }
}

export default function TrainingWindow() {
  const context = useStore();
  const [state, update] = context;
  const { trainingWindow } = state.ui;
  const { currentSchedule } = state.schedule;
  const [scheduleDone, setScheduleDone] = useState<boolean>(false);
  const [increment, setIncrement] = useState<Partial<StatState>>({});
  const [successRate, setSuccessRate] = useState<number>(0);
  const particleOverlayRef = useRef<ParticleOverlayRef>(null);

  const { difficulty, subtype, obtains } =
    currentSchedule.type !== "training"
      ? {
          difficulty: undefined,
          subtype: undefined,
          obtains: [],
        }
      : currentSchedule;

  const videoSource = subtype
    ? trainingVideoSourceMap[subtype][difficulty || 0]
    : undefined;

  const backgroundSource = subtype
    ? trainingBackgroundSourceMap[subtype]
    : undefined;

  useEffect(() => {
    if (!trainingWindow) {
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
  }, [trainingWindow]);

  return (
    <Dialog fullScreen open={trainingWindow} style={{ position: "relative" }}>
      <ParticleOverlay ref={particleOverlayRef} />
      <div
        style={{
          backgroundImage: `url(${backgroundSource})`,
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
                {getDescription(subtype, difficulty)}
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
        <ScheduleVideo source={videoSource} />
      </div>
    </Dialog>
  );
}
