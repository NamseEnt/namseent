import { Clip, engine, Mathu, Rect, Render, Translate } from "namui";
import { renderRows } from "../../common/renderRows";
import { TimelineState } from "../../timeline/type";
import { loadSequenceTitles } from "../operations/loadSequenceTitles";
import { SequenceListViewState } from "../type";
import { generateSequenceListRows } from "./generateSequenceListRows/generateSequenceListRows";
import { renderScrollbar } from "./scrollbar/renderScrollbar";

let done = false;

export const renderSequenceList: Render<
  {
    timeline: TimelineState;
    sequenceListView: SequenceListViewState;
  },
  {
    width: number;
    height: number;
  }
> = (state, props) => {
  const { sequenceListView } = state;
  const { sequenceTitles } = sequenceListView;
  const { width, height } = props;
  const spacing = 4;
  const margin = 4;
  const scrollbarWidth = 16;
  const innerWidth = width - scrollbarWidth - 2 * margin;
  if (!done) {
    loadSequenceTitles(sequenceListView);
    done = true;
  }

  const rows = generateSequenceListRows(state, {
    sequenceTitles,
    width: innerWidth,
  });
  const contentHeight = rows.reduce(
    (contentHeight, row) => contentHeight + row.height + spacing,
    0,
  );

  return [
    Rect({
      x: 0,
      y: 0,
      width,
      height,
      style: {},
      onAfterDraw: () => {
        engine.wheel.onWheel(({ deltaY }) => {
          sequenceListView.sequenceListScrollY = Mathu.clamp(
            sequenceListView.sequenceListScrollY + deltaY,
            0,
            contentHeight - height,
          );
        });
      },
    }),
    Clip(
      {
        path: new CanvasKit.Path().addRect(
          CanvasKit.XYWHRect(margin, 0, innerWidth, height),
        ),
        clipOp: CanvasKit.ClipOp.Intersect,
      },
      Translate(
        {
          x: margin,
          y: Mathu.clamp(
            -sequenceListView.sequenceListScrollY,
            height - contentHeight,
            0,
          ),
        },
        renderRows(rows, spacing),
      ),
    ),
    Translate(
      {
        x: width - scrollbarWidth,
        y: 0,
      },
      renderScrollbar(sequenceListView, {
        width: 16,
        height,
        contentHeight,
      }),
    ),
  ];
};
