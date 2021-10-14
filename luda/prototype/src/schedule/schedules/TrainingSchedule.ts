import { State } from "../../store/State";
import BaseSchedule from "./BaseSchedule";
import vocalBeginner from "../../../public/image/training/thumbnail/vocalBeginner.png";
import vocalIntermediate from "../../../public/image/training/thumbnail/vocalIntermediate.png";
import vocalAdvanced from "../../../public/image/training/thumbnail/vocalAdvanced.png";
import danceBeginner from "../../../public/image/training/thumbnail/danceBeginner.png";
import danceIntermediate from "../../../public/image/training/thumbnail/danceIntermediate.png";
import danceAdvanced from "../../../public/image/training/thumbnail/danceAdvanced.png";
import weightBeginner from "../../../public/image/training/thumbnail/weightBeginner.png";
import weightIntermediate from "../../../public/image/training/thumbnail/weightIntermediate.png";
import weightAdvanced from "../../../public/image/training/thumbnail/weightAdvanced.png";
import songWriteBeginner from "../../../public/image/training/thumbnail/songWriteBeginner.png";
import songWriteIntermediate from "../../../public/image/training/thumbnail/songWriteIntermediate.png";
import songWriteAdvanced from "../../../public/image/training/thumbnail/songWriteAdvanced.png";
import { StatState } from "../../store/State/StatState";
import constrain from "../../util/constrain";

type TrainingScheduleProps = {
  name?: string;
  duration?: number;
  editable?: boolean;
  subtype: TrainingType;
  difficulty: number;
};

export const trainingTypes = ["vocal", "songWrite", "dance", "weight"] as const;

export type TrainingType = typeof trainingTypes[number];

const thumbnailSourceMap: Record<TrainingType, Record<number, string>> = {
  vocal: {
    0: vocalBeginner,
    1: vocalIntermediate,
    2: vocalAdvanced,
  },
  dance: {
    0: danceBeginner,
    1: danceIntermediate,
    2: danceAdvanced,
  },
  weight: {
    0: weightBeginner,
    1: weightIntermediate,
    2: weightAdvanced,
  },
  songWrite: {
    0: songWriteBeginner,
    1: songWriteIntermediate,
    2: songWriteAdvanced,
  },
};

export default class TrainingSchedule extends BaseSchedule {
  readonly type = "training";
  name: string;
  duration: number;
  editable: boolean;
  subtype: TrainingType;
  difficulty: number;
  thumbnail: string;
  obtains: { success: boolean; increment: Partial<StatState> }[] = [];
  successRate: number = 0;

  constructor(props: TrainingScheduleProps) {
    super();
    this.name = props.name ?? "트레이닝";
    this.duration = props.duration ?? 1;
    this.editable = props.editable ?? true;
    this.subtype = props.subtype;
    this.difficulty = props.difficulty;
    this.thumbnail = thumbnailSourceMap[this.subtype][this.difficulty];
  }

  startSchedule(state: State): void {
    const { stat, ui } = state;
    const step = 6;
    const stepFactor = 1 / step;
    for (let i = 0; i < step; i++) {
      const success = this.didSucceed(stat);
      const appropriateDifficultyFactor =
        this.getAppropriateDifficultyFactor(stat);
      this.successRate += success ? 1 : 0;

      const obtain: TrainingSchedule["obtains"][number] = {
        success,
        increment: {},
      };
      const { increment } = obtain;
      // TODO: spend money
      switch (this.subtype) {
        case "vocal": {
          increment.vocal =
            appropriateDifficultyFactor * 1 * (success ? 1 : 0.25) * stepFactor;
          increment.mentality =
            appropriateDifficultyFactor *
            0.5 *
            (success ? 1 : 0.25) *
            stepFactor;
          increment.stress = 3 * (success ? 0.75 : 1) * stepFactor;
          increment.tiredness = 2 * (success ? 0.75 : 1) * stepFactor;
          break;
        }
        case "dance": {
          increment.dance =
            appropriateDifficultyFactor * 1 * (success ? 1 : 0.25) * stepFactor;
          increment.health =
            appropriateDifficultyFactor *
            0.75 *
            (success ? 1 : 0.25) *
            stepFactor;
          increment.stress = 4 * (success ? 0.75 : 1) * stepFactor;
          increment.tiredness = 3 * (success ? 0.75 : 1) * stepFactor;
          break;
        }
        case "weight": {
          increment.visual =
            appropriateDifficultyFactor * 1 * (success ? 1 : 0.25) * stepFactor;
          increment.health =
            appropriateDifficultyFactor *
            0.75 *
            (success ? 1 : 0.25) *
            stepFactor;
          increment.stress = 3 * (success ? 0.75 : 1) * stepFactor;
          increment.tiredness = 4 * (success ? 0.75 : 1) * stepFactor;
          break;
        }
        default: {
          increment.sense =
            appropriateDifficultyFactor * 1 * (success ? 1 : 0.25) * stepFactor;
          increment.mentality =
            appropriateDifficultyFactor *
            0.5 *
            (success ? 1 : 0.25) *
            stepFactor;
          increment.stress = 3 * (success ? 0.75 : 1) * stepFactor;
          increment.tiredness = 2 * (success ? 0.75 : 1) * stepFactor;
          break;
        }
      }
      this.obtains.push(obtain);
    }
    ui.trainingWindow = true;
  }

  endSchedule(state: State): void {
    state.ui.trainingWindow = false;
  }

  clone() {
    const newInstance = new TrainingSchedule({
      name: this.name,
      subtype: this.subtype,
      difficulty: this.difficulty,
    });
    newInstance.duration = this.duration;
    newInstance.editable = this.editable;
    return newInstance;
  }

  didSucceed(stat: StatState) {
    const { mentality, stress, health, tiredness, will } = stat;
    const mentalityFactor = constrain(
      (mentality - stress + mentality / 4) / mentality,
    );
    const healthFactor = constrain((health - tiredness + health / 4) / health);
    return (
      Math.random() <
      Math.min(mentalityFactor, healthFactor) / 2 + will / 100 / 2
    );
  }

  getAppropriateDifficultyFactor(stat: StatState) {
    const targetStatMap: Record<TrainingType, number> = {
      vocal: stat.vocal,
      dance: stat.dance,
      weight: stat.health,
      songWrite: stat.sense,
    };
    const targetStat = targetStatMap[this.subtype] / 100;
    return Math.max(
      -12 * (targetStat - 1 / 6 + (1 / 3) * this.difficulty) ** 2 + 1,
      0,
    );
  }
}
