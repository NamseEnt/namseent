import { RenderingTree } from "../type";
import { Rect } from "./Rect";

export function Button(
  param: Parameters<typeof Rect>[0] & {
    content?: RenderingTree;
    onClick: () => void;
  }
): RenderingTree {
  return [
    Rect({
      ...param,
    }),
    param.content,
  ];
}
