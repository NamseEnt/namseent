import { RenderingTree } from "namui";
import {
  CommunicationState,
  renderCommunication,
} from "./game/communication/renderCommunication";

type State = {
  counters: {
    count: number;
  }[];
  communication: CommunicationState;
};

export function render(state: State): RenderingTree {
  return renderCommunication(state.communication);
}
