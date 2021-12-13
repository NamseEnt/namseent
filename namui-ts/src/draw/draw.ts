import {
  DrawCommand,
  EngineContext,
  RenderingData,
  RenderingTree,
  Vector,
} from "../type";
import { drawImage } from "./image/drawImage";
import { drawText } from "./drawText";
import {
  AfterDrawCommand,
  ClipCommand,
  TranslateCommand,
} from "../types/SpecialRenderingCommand";

export function draw(
  engineContext: EngineContext,
  renderingTree: RenderingTree,
): void {
  if (!(renderingTree instanceof Array)) {
    renderingTree = [renderingTree];
  }
  renderingTree.map((element) => {
    if (!element) {
      return;
    }
    if (element instanceof Array) {
      return draw(engineContext, element);
    }
    if ("type" in element) {
      switch (element.type) {
        case "translate":
          return onTranslate(engineContext, element);
        case "afterDraw":
          return onAfterDraw(engineContext, element);
        case "clip":
          return onClip(engineContext, element);
        default:
          return;
      }
    }
    return drawRenderingData(engineContext, element);
  });
}

function drawRenderingData(
  engineContext: EngineContext,
  renderingData: RenderingData,
): void {
  renderingData.drawCalls.forEach((drawCall) => {
    drawCall.commands.forEach((command) => {
      drawCommand(engineContext, command);
    });
  });
}

function drawCommand(engineContext: EngineContext, command: DrawCommand): void {
  switch (command.type) {
    case "path":
      return engineContext.canvas.drawPath(command.path, command.paint);
    case "image":
      return drawImage(engineContext, command);
    case "text":
      return drawText(engineContext, command);
    default:
      console.error("Unknown command", command);
      throw new Error(`Unknown command ${command}`);
  }
}

function onTranslate(
  engineContext: EngineContext,
  translateCommand: TranslateCommand,
): void {
  engineContext.canvas.translate(translateCommand.x, translateCommand.y);

  draw(engineContext, translateCommand.renderingTree);

  engineContext.canvas.translate(-translateCommand.x, -translateCommand.y);
}

function onAfterDraw(
  engineContext: EngineContext,
  afterDrawCommand: AfterDrawCommand,
): void {
  const matrix4x4 = engineContext.canvas.getLocalToDevice();
  const x = engineContext.canvasKit.M44.rc(matrix4x4, 0, 3);
  const y = engineContext.canvasKit.M44.rc(matrix4x4, 1, 3);
  afterDrawCommand.callback({
    translated: new Vector(x, y),
  });
}

function onClip(engineContext: EngineContext, clipCommand: ClipCommand): void {
  // TODO : handle previous clip before this clip command. maybe should merge?

  const { canvas } = engineContext;
  canvas.save();

  const { path, clipOp } = clipCommand;
  canvas.clipPath(path, clipOp, true);

  draw(engineContext, clipCommand.renderingTree);

  canvas.restore();
}
