import RAFManager from "raf-manager";
import React, {
  useRef,
  useEffect,
  useState,
  useImperativeHandle,
  forwardRef,
} from "react";
import ParticleManager from "../EvaluationWindow/GameManager/ParticleManager";

export type ParticleOverlayRef = {
  particleManager?: ParticleManager;
  canvasSize: {
    width: number;
    height: number;
  };
};

const ParticleOverlay = forwardRef<ParticleOverlayRef>((props, ref) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [particleManager, setParticleManager] = useState<ParticleManager>();
  const [startedAt] = useState<number>(Date.now());

  useImperativeHandle(ref, () => ({
    particleManager,
    canvasSize: {
      width: canvasRef.current?.scrollWidth || 0,
      height: canvasRef.current?.scrollHeight || 0,
    },
  }));

  useEffect(() => {
    const particleManager = new ParticleManager();
    setParticleManager(particleManager);
    const resize = () => {
      const canvas = canvasRef.current;
      if (!canvas) {
        return;
      }
      canvas.width = canvas.scrollWidth;
      canvas.height = canvas.scrollHeight;
    };
    const render = () => {
      const context = canvasRef.current?.getContext("2d");
      if (!context) {
        return;
      }
      resize();
      particleManager.tick(Date.now() - startedAt);
      particleManager.render(context);
    };

    RAFManager.add(render, -1);
    return () => {
      RAFManager.remove(render);
    };
  }, []);

  return (
    <canvas
      style={{
        position: "absolute",
        top: 0,
        right: 0,
        bottom: 0,
        left: 0,
        pointerEvents: "none",
        width: "100%",
        height: "100%",
      }}
      ref={canvasRef}
    />
  );
});

export default ParticleOverlay;
