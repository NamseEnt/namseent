import { getImageElement } from "../../../../util/getImageElement";
import { EvaluationRenderingContext, Turn } from "../type";
import hayeonFace from "../../../../../public/image/evaluation-simon/hayeon-face.png";
import trainerFace from "../../../../../public/image/evaluation-simon/trainer-face.png";

const faceImageSourceMap: Record<Turn, HTMLImageElement> = {
  hayeon: getImageElement(hayeonFace),
  trainer: getImageElement(trainerFace),
  gameover: new Image(),
};

function drawFace(
  context: CanvasRenderingContext2D,
  turn: Turn,
  alpha: number,
  size: number,
) {
  const imageSource = faceImageSourceMap[turn];
  context.save();
  context.globalAlpha = alpha;
  context.drawImage(imageSource, -size / 2, -size / 2, size, size);
  context.restore();
}

function getAlpha(timeDiff: number) {
  const fadeTime = 333;
  if (timeDiff > fadeTime) {
    return 1;
  }
  const progress = timeDiff / fadeTime;
  return progress;
}

export default function renderFace(
  renderingContext: EvaluationRenderingContext,
) {
  const { context, currentTime, canvasSize, unitSize, turn, turnChangedAt } =
    renderingContext;

  const newAlpha = getAlpha(currentTime - turnChangedAt);
  const oldAlpha = 1 - newAlpha;
  const faceSize = 35 * unitSize;

  context.save();
  context.translate(canvasSize.width / 2, canvasSize.height / 2);
  switch (turn) {
    case "hayeon": {
      drawFace(context, "trainer", oldAlpha, faceSize);
      drawFace(context, "hayeon", newAlpha, faceSize);
      break;
    }
    case "trainer": {
      drawFace(context, "hayeon", oldAlpha, faceSize);
      drawFace(context, "trainer", newAlpha, faceSize);
      break;
    }
    default: {
      break;
    }
  }

  context.restore();
}
