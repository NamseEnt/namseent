import { Vector } from "../..";

export interface IRenderManager {
  isGlobalVectorOutOfRenderingData(
    globalVector: Vector,
    renderingDataId: string,
  ): boolean;
}
