import { IEngine, IEngineInternal } from "./IEngine";
import { webEngine } from "./webEngine";

export const engine: IEngine = webEngine;
export const engineInternal: IEngineInternal = webEngine;
