import { Day, Time, Week } from "./type";

function convertIndexToWeek(week: number) {
  switch (week) {
    case 0: {
      return Week.First;
    }
    case 1: {
      return Week.Second;
    }
    case 2: {
      return Week.Third;
    }
    case 3: {
      return Week.Fourth;
    }
    case 4: {
      return Week.Fifth;
    }
    case 5: {
      return Week.Sixth;
    }
    case 6: {
      return Week.Seventh;
    }
    case 7: {
      return Week.Eighth;
    }
    case 8: {
      return Week.Ninth;
    }
    case 9: {
      return Week.Tenth;
    }
    case 10: {
      return Week.Eleventh;
    }
    case 11: {
      return Week.Twelfth;
    }
    case 12: {
      return Week.Thirteenth;
    }
    default: {
      throw new Error("Invalid input. Week should be 0 ~ 12 int");
    }
  }
}

function convertIndexToDay(day: number) {
  switch (day) {
    case 0: {
      return Day.Monday;
    }
    case 1: {
      return Day.Tuesday;
    }
    case 2: {
      return Day.Wednesday;
    }
    case 3: {
      return Day.Thursday;
    }
    case 4: {
      return Day.Friday;
    }
    case 5: {
      return Day.Saturday;
    }
    case 6: {
      return Day.Sunday;
    }
    default: {
      throw new Error("Invalid input. Day should be 0 ~ 6 int");
    }
  }
}

function convertIndexToTime(time: number) {
  switch (time) {
    case 0: {
      return Time.A;
    }
    case 1: {
      return Time.B;
    }
    case 2: {
      return Time.C;
    }
    case 3: {
      return Time.D;
    }
    case 4: {
      return Time.E;
    }
    case 5: {
      return Time.F;
    }
    case 6: {
      return Time.G;
    }
    case 7: {
      return Time.H;
    }
    default: {
      throw new Error("Invalid input. Time should be 0 ~ 7 int");
    }
  }
}

export default function convertTimeToDayTime(totalTime: number) {
  let remainedTime = totalTime;
  const weekIndex = Math.floor(remainedTime / 56);
  remainedTime -= weekIndex * 56;
  const dayIndex = Math.floor(remainedTime / 8);
  remainedTime -= dayIndex * 8;
  const timeIndex = remainedTime;

  return [
    convertIndexToWeek(weekIndex),
    convertIndexToDay(dayIndex),
    convertIndexToTime(timeIndex),
  ] as [Week, Day, Time];
}
