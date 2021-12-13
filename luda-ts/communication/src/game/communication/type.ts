import { Language } from "namui";

export type Shot = {
  image: {
    url: string;
    position: {
      x: number;
      y: number;
    };
    size: {
      width: number;
      height: number;
    };
  };
  subtitle: {
    [language in Language]: string;
  };
  audio?: {
    url: string;
    range: {
      startMs: number;
      endMs: number;
    };
  };
  durationMs: number;
};
