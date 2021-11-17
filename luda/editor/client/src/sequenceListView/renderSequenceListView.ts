import {
  BorderPosition,
  Clip,
  ColorUtil,
  Rect,
  Render,
  Translate,
} from "namui";
import { renderRows } from "../common/renderRows";
import { PlayerScreen } from "../livePlayer/playerScreen/PlayerScreen";
import { TimelineState } from "../timeline/type";
import { renderEditingSequenceTitle } from "./renderEditingSequenceTitle";
import { renderSelectedSequenceTitle } from "./renderSelectedSequenceTitle";
import { renderSequenceAddButton } from "./renderSequenceAddButton";
import { renderSequenceIndexFetchButton } from "./renderSequenceIndexFetchButton";
import { renderSequencePreview } from "./renderSequencePreview";
import { renderSequenceAddDialog } from "./sequenceAddDialog/renderSequenceAddDialog";
import { renderSequenceList } from "./sequenceList/renderSequenceList";
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
  const spacing = 4;

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
                  height: 24,
                  renderingData: renderEditingSequenceTitle(
                    {},
                    { title: sequenceListView.editingSequenceTitle },
                  ),
                },
                {
                  height: 24,
                  renderingData: renderSelectedSequenceTitle(
                    {},
                    { title: sequenceListView.preloadedSequence?.title },
                  ),
                },
                {
                  height: 128,
                  renderingData: renderSequencePreview(sequenceListView, {
                    width,
                    height: 128,
                  }),
                },
                {
                  height: 36,
                  renderingData: renderSequenceAddButton(sequenceListView, {
                    width,
                  }),
                },
                {
                  height: 36,
                  renderingData: renderSequenceIndexFetchButton(
                    sequenceListView,
                    {
                      width,
                    },
                  ),
                },
                {
                  height: 0,
                  renderingData: renderSequenceList(state, {
                    width: width,
                    height: height - (24 + 24 + 128 + 36 + 36 + 5 * spacing),
                  }),
                },
              ],
              spacing,
            ),
      ),
    ],
  );
};
