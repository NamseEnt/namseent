import { ClipOp, Path } from "canvaskit-wasm";
import { RenderingTree, Vector } from "../type";

export type TranslateCommand = {
  type: "translate";
  x: number;
  y: number;
  renderingTree: RenderingTree;
};

export type AfterDrawCallback = (param: { translated: Vector }) => void;

export type AfterDrawCommand = {
  type: "afterDraw";
  callback: AfterDrawCallback;
};

export type ClipCommand = {
  type: "clip";
  path: Path;
  clipOp: ClipOp;
  renderingTree: RenderingTree;
};

export type SpecialRenderingCommand =
  | TranslateCommand
  | AfterDrawCommand
  | ClipCommand;
