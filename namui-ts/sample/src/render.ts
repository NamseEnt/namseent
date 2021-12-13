import { RenderingTree, Selection, Translate } from "namui";
import { renderWithMouseEvent } from "./renderWithMouseEvent";

type State = {
  text: string;
  focus: boolean;
  selection?: Selection;
};

export function render(state: State): RenderingTree {
  return [renderWithMouseEvent()];
}
