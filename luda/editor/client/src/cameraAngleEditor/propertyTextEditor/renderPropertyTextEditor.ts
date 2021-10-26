import { RenderingTree, Translate } from "namui";
import { CameraAngleEditorState } from "../type";
import { renderSingleTextEditorRow } from "./renderSingleTextEditorRow";
import { renderXywhRectTextEditorRows } from "./renderXywhRectTextEditorRows";

export type Row = RenderingTree;

export function renderPropertyTextEditor(
  state: CameraAngleEditorState,
): RenderingTree {
  const gap = undefined;

  const rows: Row[] = [
    renderSingleTextEditorRow(
      {
        label: "imageSourceUrl",
        value: state.cameraAngle.imageSourceUrl,
        onChange: (value: string) => {
          state.cameraAngle.imageSourceUrl = value;
        },
        textInputId: "imageSourceUrl",
      },
      {
        textInput: state.propertyTextEditor.textInput,
      },
    ),
    gap,
    ...renderXywhRectTextEditorRows(
      {
        label: "sourceRect",
      },
      {
        rect: state.cameraAngle.sourceRect,
        textInput: state.propertyTextEditor.textInput,
      },
    ),
    gap,
    ...renderXywhRectTextEditorRows(
      {
        label: "destRect",
      },
      {
        rect: state.cameraAngle.destRect,
        textInput: state.propertyTextEditor.textInput,
      },
    ),
  ];

  return [
    ...rows.map((row, index) => {
      return Translate(
        {
          x: 0,
          y: 20 * index,
        },
        row,
      );
    }),
  ];
}
