import { State } from "../../store/State";
import BaseSchedule from "./BaseSchedule";
import error from "../../../public/image/error.png";
import { StatState } from "../../store/State/StatState";
import pickRandomItem from "../../util/pickRandomItem";

type BreakScheduleProps = {
  name: string;
  duration?: number;
  editable?: boolean;
};

export const breakTypes = [
  "walk",
  "cafe",
  "game",
  "reading",
  "nap",
  "watching",
] as const;

export type BreakType = typeof breakTypes[number];

export default class BreakSchedule extends BaseSchedule {
  readonly type = "break";
  name: string;
  duration: number;
  editable: boolean;
  subtype: BreakType;
  thumbnail = error;
  obtains: Partial<StatState>[] = [];
  successRate: number = 0;

  constructor(props: BreakScheduleProps) {
    super();
    this.name = props.name;
    this.duration = props.duration ?? 1;
    this.editable = props.editable ?? true;
    this.subtype = "walk";
  }

  startSchedule(state: State): void {
    const { ui } = state;
    const step = 6;
    const stepFactor = 1 / step;
    this.subtype = pickRandomItem(breakTypes);
    for (let i = 0; i < step; i++) {
      const obtain: BreakSchedule["obtains"][number] = {};
      // TODO: spend money
      switch (this.subtype) {
        case "walk": {
          obtain.health = 0.25 * stepFactor;
          obtain.will = 0.5 * stepFactor;
          obtain.stress = -2 * stepFactor;
          break;
        }
        case "cafe": {
          obtain.visual = 0.25 * stepFactor;
          obtain.health = -0.25 * stepFactor;
          obtain.will = 0.5 * stepFactor;
          obtain.stress = -3 * stepFactor;
          break;
        }
        case "game": {
          obtain.visual = -0.25 * stepFactor;
          obtain.sense = 0.5 * stepFactor;
          obtain.mentality = -0.75 * stepFactor;
          obtain.health = -0.5 * stepFactor;
          obtain.stress = -5 * stepFactor;
          break;
        }
        case "reading": {
          obtain.visual = -0.25 * stepFactor;
          obtain.mentality = 0.5 * stepFactor;
          obtain.health = -0.5 * stepFactor;
          obtain.tiredness = -3 * stepFactor;
          break;
        }
        case "nap": {
          obtain.visual = -0.5 * stepFactor;
          obtain.mentality = -0.75 * stepFactor;
          obtain.health = -1 * stepFactor;
          obtain.stress = -4 * stepFactor;
          obtain.tiredness = -5 * stepFactor;
          break;
        }
        default: {
          obtain.sense = 0.5 * stepFactor;
          obtain.mentality = -0.5 * stepFactor;
          obtain.health = -0.5 * stepFactor;
          obtain.stress = -3 * stepFactor;
          obtain.tiredness = -3 * stepFactor;
          break;
        }
      }
      this.obtains.push(obtain);
    }
    ui.breakWindow = true;
  }

  endSchedule(state: State): void {
    state.ui.breakWindow = false;
  }

  clone() {
    const newInstance = new BreakSchedule({
      name: this.name,
    });
    newInstance.duration = this.duration;
    newInstance.editable = this.editable;
    return newInstance;
  }
}
