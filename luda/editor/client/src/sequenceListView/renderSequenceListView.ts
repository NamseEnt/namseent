import {
  BorderPosition,
  Clip,
  ColorUtil,
  Rect,
  Render,
  Translate,
} from "namui";
import { renderRows } from "../common/renderRows";
import { TimelineState } from "../timeline/type";
import { renderSequenceAddButton } from "./renderSequenceAddButton";
import { renderSequenceName } from "./renderSequenceName";
import { renderSequenceAddDialog } from "./sequenceAddDialog/renderSequenceAddDialog";
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
        sequenceListView.addingSequence
          ? renderSequenceAddDialog(state, { width })
          : renderRows(
              [
                {
                  height: 36,
                  renderingData: renderSequenceAddButton(sequenceListView, {
                    width,
                  }),
                },
                {
                  height: 24,
                  renderingData: renderSequenceName(
                    {},
                    { sequenceName: sequenceListView.editingFileName },
                  ),
                },
              ],
              4,
            ),
      ),
    ],
  );
};
