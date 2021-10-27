import { RenderingTree } from "..";

export interface IManagerInternal {
  resetBeforeRender?: () => void;
  destroy?: () => void;
  afterRender?: (renderingTree: RenderingTree) => void;
}
