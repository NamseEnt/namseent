import { Day, Time, Week } from "./type";

function convertWeekToIndex(week: Week) {
  switch (week) {
    case Week.First: {
      return 0;
    }
    case Week.Second: {
      return 1;
    }
    case Week.Third: {
      return 2;
    }
    case Week.Fourth: {
      return 3;
    }
    case Week.Fifth: {
      return 4;
    }
    case Week.Sixth: {
      return 5;
    }
    case Week.Seventh: {
      return 6;
    }
    case Week.Eighth: {
      return 7;
    }
    case Week.Ninth: {
      return 8;
    }
    case Week.Tenth: {
      return 9;
    }
    case Week.Eleventh: {
      return 10;
    }
    case Week.Twelfth: {
      return 11;
    }
    case Week.Thirteenth: {
      return 12;
    }
  }
}

function convertDayToIndex(day: Day) {
  switch (day) {
    case Day.Monday: {
      return 0;
    }
    case Day.Tuesday: {
      return 1;
    }
    case Day.Wednesday: {
      return 2;
    }
    case Day.Thursday: {
      return 3;
    }
    case Day.Friday: {
      return 4;
    }
    case Day.Saturday: {
      return 5;
    }
    case Day.Sunday: {
      return 6;
    }
  }
}

function convertTimeToIndex(time: Time) {
  switch (time) {
    case Time.A: {
      return 0;
    }
    case Time.B: {
      return 1;
    }
    case Time.C: {
      return 2;
    }
    case Time.D: {
      return 3;
    }
    case Time.E: {
      return 4;
    }
    case Time.F: {
      return 5;
    }
    case Time.G: {
      return 6;
    }
    case Time.H: {
      return 7;
    }
  }
}

export default function convertDayTimeToTime(week: Week, day: Day, time: Time) {
  const weekIndex = convertWeekToIndex(week);
  const dayIndex = convertDayToIndex(day);
  const timeIndex = convertTimeToIndex(time);
  return weekIndex * 56 + dayIndex * 8 + timeIndex;
}
