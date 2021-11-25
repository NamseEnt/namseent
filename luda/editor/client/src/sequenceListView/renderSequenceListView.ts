import { Clip, engine, Render, Translate } from "namui";
import { renderRows } from "../common/renderRows";
import { TimelineState } from "../timeline/type";
import { renderSequenceAddButton } from "./renderSequenceAddButton";
import { renderSequenceIndexReloadButton } from "./renderSequenceIndexReloadButton";
import { renderSequencePreview } from "./renderSequencePreview";
import { renderSequenceAddDialog as renderSequenceTitleInputDialog } from "./sequenceTitleInputDialog/renderSequenceTitleInputDialog";
import { renderSequenceList } from "./sequenceList/renderSequenceList";
import { SequenceListViewActionState, SequenceListViewState } from "./type";
import { checkLoadingTimeout } from "./checkLoadingTimeout";
import { renderLoadingPage } from "./renderLoadingPage/renderLoadingPage";

export const renderSequenceListView: Render<
  {
    timeline: TimelineState;
    sequenceListView: SequenceListViewState;
  },
  {}
> = (state, props) => {
  const { sequenceListView } = state;

  const now = Date.now();
  const loadingTimeoutMs = 5000;
  checkLoadingTimeout({
    state: sequenceListView.loadingSequence,
    now,
    timeoutMs: loadingTimeoutMs,
  });
  checkLoadingTimeout({
    state: sequenceListView.preloadedSequence,
    now,
    timeoutMs: loadingTimeoutMs,
  });

  const margin = 8;
  const spacing = 4;
  const listWidth = sequenceListView.layout.listWidth - margin;
  const previewWidth =
    sequenceListView.layout.rect.width -
    sequenceListView.layout.listWidth -
    margin -
    spacing;
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
      Translate(
        {
          x: margin,
          y: margin,
        },
        [
          sequenceListView.actionState !== SequenceListViewActionState.none
            ? renderSequenceTitleInputDialog(state, { width: listWidth })
            : renderRows(
                [
                  {
                    height: 36,
                    renderingData: renderSequenceAddButton(sequenceListView, {
                      width: listWidth,
                    }),
                  },
                  {
                    height: 36,
                    renderingData: renderSequenceIndexReloadButton(
                      sequenceListView,
                      {
                        width: listWidth,
                      },
                    ),
                  },
                  {
                    height: 0,
                    renderingData: renderSequenceList(state, {
                      width: listWidth,
                      height: height - (36 + 36 + 2 * spacing),
                    }),
                  },
                ],
                spacing,
              ),
        ],
      ),
      Translate(
        {
          x: state.sequenceListView.layout.listWidth + spacing,
          y: 0,
        },
        renderSequencePreview(sequenceListView, {
          width: previewWidth,
          height,
        }),
      ),
      renderLoadingPage(sequenceListView, sequenceListView.layout.rect),
    ],
  );
};
