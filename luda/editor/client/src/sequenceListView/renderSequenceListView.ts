import {
  BorderPosition,
  Clip,
  ColorUtil,
  FontWeight,
  Language,
  Rect,
  Render,
  Text,
  TextAlign,
  TextBaseline,
  Translate,
} from "namui";
import { TimelineState } from "../timeline/type";
import { SequenceListViewState } from "./type";

export const renderSequenceListView: Render<
  {
    timeline: TimelineState;
    sequenceListView: SequenceListViewState;
  },
  {}
> = (state, props) => {
  const { sequenceListView } = state;

  const borderWidth = 1;
  const margin = 8;
  const width = sequenceListView.layout.rect.width - 2 * margin;
  const height = sequenceListView.layout.rect.height - 2 * margin;

  return Clip(
    {
      path: new CanvasKit.Path().addRect(
        CanvasKit.XYWHRect(
          sequenceListView.layout.rect.x,
          sequenceListView.layout.rect.y,
          sequenceListView.layout.rect.width,
          sequenceListView.layout.rect.height,
        ),
      ),
      clipOp: CanvasKit.ClipOp.Intersect,
    },
    [
      Rect({
        ...sequenceListView.layout.rect,
        style: {
          stroke: {
            color: ColorUtil.Black,
            width: borderWidth,
            borderPosition: BorderPosition.inside,
          },
        },
      }),
      Translate(
        {
          x: margin,
          y: margin,
        },
        [
          Text({
            x: 0,
            y: 0,
            align: TextAlign.left,
            baseline: TextBaseline.top,
            fontType: {
              language: Language.ko,
              serif: false,
              fontWeight: FontWeight.regular,
              size: 20,
            },
            style: {
              color: ColorUtil.Black,
            },
            text: "SequenceName: ",
          }),
          Text({
            x: 160,
            y: 0,
            align: TextAlign.left,
            baseline: TextBaseline.top,
            fontType: {
              language: Language.ko,
              serif: false,
              fontWeight: FontWeight.regular,
              size: 20,
            },
            style: {
              color: sequenceListView.editingFileName
                ? ColorUtil.Black
                : ColorUtil.Red,
            },
            text:
              sequenceListView.editingFileName ||
              "No name has been specified. Changes will not be saved.",
          }),
        ],
      ),
    ],
  );
};
