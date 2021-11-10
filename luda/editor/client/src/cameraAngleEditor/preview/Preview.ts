import { ColorUtil, Rect, Render, Translate, BorderPosition } from "namui";
import { CameraClip } from "../../livePlayer/playerScreen/camera/CameraClip";
import { CameraAngleEditorState } from "../type";

export const Preview: Render<CameraAngleEditorState> = (
  state: CameraAngleEditorState,
) => {
  return [
    Translate(
      {
        ...state.layout.sub.preview,
      },
      [
        Rect({
          ...state.layout.sub.preview,
          x: 0,
          y: 0,
          style: {
            stroke: {
              color: ColorUtil.Black,
              width: 1,
              borderPosition: BorderPosition.inside,
            },
          },
        }),
        CameraClip(
          {},
          {
            cameraAngle: state.cameraAngle,
            whSize: state.layout.sub.preview,
          },
        ),
      ],
    ),
  ];
};
