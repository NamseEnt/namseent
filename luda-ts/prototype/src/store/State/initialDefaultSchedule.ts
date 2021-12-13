import BreakSchedule from "../../schedule/schedules/BreakSchedule";
import EatMealSchedule from "../../schedule/schedules/EatMealSchedule";
import ManageYoutubeSchedule from "../../schedule/schedules/ManageYoutubeSchedule";
import OffSchedule from "../../schedule/schedules/OffSchedule";
import TrainingSchedule from "../../schedule/schedules/TrainingSchedule";
import { SchedulePlan } from "../../schedule/type";

export const initialDefaultSchedule: SchedulePlan = {
  First: {
    Monday: {
      A: new TrainingSchedule({ subtype: "weight", difficulty: 0 }),
      B: new EatMealSchedule({ name: "점심식사" }),
      C: new BreakSchedule({ name: "자유시간" }),
      D: new TrainingSchedule({ subtype: "vocal", difficulty: 0 }),
      E: new TrainingSchedule({ subtype: "vocal", difficulty: 0 }),
      F: new EatMealSchedule({ name: "저녁식사" }),
      G: new BreakSchedule({ name: "자유시간" }),
      H: new TrainingSchedule({ subtype: "vocal", difficulty: 0 }),
    },
    Tuesday: {
      A: new TrainingSchedule({ subtype: "weight", difficulty: 0 }),
      B: new EatMealSchedule({ name: "점심식사" }),
      C: new BreakSchedule({ name: "자유시간" }),
      D: new TrainingSchedule({ subtype: "vocal", difficulty: 0 }),
      E: new TrainingSchedule({ subtype: "vocal", difficulty: 0 }),
      F: new EatMealSchedule({ name: "저녁식사" }),
      G: new BreakSchedule({ name: "자유시간" }),
      H: new TrainingSchedule({ subtype: "vocal", difficulty: 0 }),
    },
    Wednesday: {
      A: new TrainingSchedule({ subtype: "weight", difficulty: 0 }),
      B: new EatMealSchedule({ name: "점심식사" }),
      C: new BreakSchedule({ name: "자유시간" }),
      D: new TrainingSchedule({ subtype: "vocal", difficulty: 0 }),
      E: new TrainingSchedule({ subtype: "vocal", difficulty: 0 }),
      F: new EatMealSchedule({ name: "저녁식사" }),
      G: new BreakSchedule({ name: "자유시간" }),
      H: new TrainingSchedule({ subtype: "vocal", difficulty: 0 }),
    },
    Thursday: {
      A: new TrainingSchedule({ subtype: "weight", difficulty: 0 }),
      B: new EatMealSchedule({ name: "점심식사" }),
      C: new BreakSchedule({ name: "자유시간" }),
      D: new TrainingSchedule({ subtype: "vocal", difficulty: 0 }),
      E: new TrainingSchedule({ subtype: "vocal", difficulty: 0 }),
      F: new EatMealSchedule({ name: "저녁식사" }),
      G: new BreakSchedule({ name: "자유시간" }),
      H: new TrainingSchedule({ subtype: "vocal", difficulty: 0 }),
    },
    Friday: {
      A: new TrainingSchedule({ subtype: "weight", difficulty: 0 }),
      B: new EatMealSchedule({ name: "점심식사" }),
      C: new BreakSchedule({ name: "자유시간" }),
      D: new TrainingSchedule({ subtype: "vocal", difficulty: 0 }),
      E: new TrainingSchedule({ subtype: "vocal", difficulty: 0 }),
      F: new EatMealSchedule({ name: "저녁식사" }),
      G: new BreakSchedule({ name: "자유시간" }),
      H: new TrainingSchedule({ subtype: "vocal", difficulty: 0 }),
    },
    Saturday: {
      A: new ManageYoutubeSchedule({ name: "유튜브 관리", duration: 8 }),
    },
    Sunday: {
      A: new OffSchedule({ name: "휴일", duration: 8 }),
    },
  },
};
