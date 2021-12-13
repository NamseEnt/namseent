import {
  ColorUtil,
  Language,
  Image,
  Text,
  RenderingTree,
  TextAlign,
  TextBaseline,
} from "namui";
import { shots } from "./shots";

export type CommunicationState = {
  shotIndex: number;
  language: Language;
};

export function renderCommunication(
  communicationState: CommunicationState,
): RenderingTree {
  const { shotIndex, language } = communicationState;
  const shot = shots[shotIndex];
  if (!shot) {
    return;
  }
  const subtitle = shot.subtitle[language];
  if (!subtitle) {
    throw new Error(`No subtitle for language ${language}`);
  }

  return [
    Image({
      ...shot.image,
      url: shot.image.url,
    }),
    Text({
      text: subtitle,
      x: 1920 / 2,
      y: 1080 * (3 / 4) - 80,
      align: TextAlign.center,
      baseline: TextBaseline.top,
      fontType: {
        serif: false,
        size: 64,
        language,
      },
      style: {
        border: {
          color: ColorUtil.Black,
          width: 1.5,
        },
        dropShadow: {
          x: 1,
          y: 3,
          color: ColorUtil.Black,
        },
        color: ColorUtil.White,
        background: {
          color: ColorUtil.Color01(0, 0, 0, 0.5),
          margin: {
            left: 5,
            right: 5,
            top: 5,
            bottom: 5,
          },
        },
      },
    }),
  ];
}
