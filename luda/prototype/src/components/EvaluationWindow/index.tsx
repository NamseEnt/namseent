import { Dialog } from "@material-ui/core";
import React, { useEffect, useRef, useState } from "react";
import RAFManager from "raf-manager";
import endSchedule from "../../store/Action/schedule/endSchedule";
import useStore from "../../store/useStore";
import stage from "../../../public/image/stage.jpg";
import GameManager from "./GameManager";

export default function EvaluationWindow() {
  const context = useStore();
  const [state, update] = context;
  const { evaluationWindow } = state.ui;
  const [fps, setFps] = useState<number>(0);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    if (!evaluationWindow) {
      return;
    }
    let fpsCounter = 0;
    const gameManager = new GameManager({
      gameFinishHandler: () => update((state) => endSchedule(state)),
      volume: 0.3,
      maxStep: 5,
      keyIncreasePerStep: 1,
      initialKeyCount: 3,
    });
    const resize = () => {
      const canvas = canvasRef.current;
      if (!canvas) {
        return;
      }

      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
      canvas.style.width = `${window.innerWidth}px`;
      canvas.style.height = `${window.innerHeight}px`;
    };
    const render = () => {
      fpsCounter += 1;
      const context = canvasRef.current?.getContext("2d");
      if (!context) {
        return;
      }
      resize();
      gameManager.render(context);
    };

    RAFManager.add(render, -1);
    const fpsCounterId = setInterval(() => {
      setFps(fpsCounter);
      fpsCounter = 0;
    }, 1000);
    window.addEventListener("keydown", gameManager.keyDownHandler);
    return () => {
      RAFManager.remove(render);
      clearInterval(fpsCounterId);
      window.removeEventListener("keydown", gameManager.keyDownHandler);
    };
  }, [evaluationWindow]);

  return (
    <Dialog fullScreen open={evaluationWindow}>
      <canvas
        style={{
          backgroundImage: `url(${stage})`,
          backgroundSize: "cover",
          backgroundPosition: "50% 50%",
        }}
        ref={canvasRef}
      />
      <div
        style={{
          color: "white",
          backgroundColor: "rgba(0, 0, 0, 0.3)",
          padding: "0.2em",
          fontSize: "1em",
          position: "fixed",
          left: 0,
          top: 0,
        }}
      >
        {fps}
      </div>
    </Dialog>
  );
}
