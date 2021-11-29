import { Clip, engine, Render, Translate } from "namui";
import { renderRows } from "../common/renderRows";
import { TimelineSequenceNullableState } from "../timeline/type";
import { renderSequenceAddButton } from "./renderSequenceAddButton";
import { renderSequenceIndexReloadButton } from "./renderSequenceIndexReloadButton";
import { renderSequencePreview } from "./renderSequencePreview";
import { renderSequenceAddDialog as renderSequenceTitleInputDialog } from "./sequenceTitleInputDialog/renderSequenceTitleInputDialog";
import { renderSequenceList } from "./sequenceList/renderSequenceList";
import { SequenceListViewActionState, SequenceListViewState } from "./type";
import { renderLoadingPage } from "./renderLoadingPage/renderLoadingPage";
import { loadSequence } from "./operations/loadSequence";

export const renderSequenceListView: Render<
  {
    timeline: TimelineSequenceNullableState;
    sequenceListView: SequenceListViewState;
  },
  {}
> = (state, props) => {
  const { sequenceListView, timeline } = state;

  if (sequenceListView.loadingSequence) {
    sequenceListView.loadingSequence.state = loadSequence(
      sequenceListView.loadingSequence,
    );
  }

  if (sequenceListView.preloadedSequence) {
    sequenceListView.preloadedSequence.state = loadSequence(
      sequenceListView.preloadedSequence,
    );

    // shouldCalcSequenceLength
    if (
      sequenceListView.preloadedSequence.state.type === "loaded" &&
      typeof sequenceListView.preloadedSequence.lengthMs === "undefined"
    ) {
      sequenceListView.preloadedSequence.lengthMs =
        sequenceListView.preloadedSequence.state.tracks.reduce(
          (trackLength, track) =>
            Math.max(
              trackLength,
              track.clips.reduce(
                (clipLength, clip) => Math.max(clipLength, clip.endMs),
                0,
              ),
            ),
          0,
        );
    }
  }

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
      renderLoadingPage(
        {
          sequenceListViewState: sequenceListView,
          timelineState: timeline,
        },
        sequenceListView.layout.rect,
      ),
    ],
  );
};
