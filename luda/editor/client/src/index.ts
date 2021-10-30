import { ColorUtil, startEngine } from "namui";
import { render } from "./render";

startEngine(
  {
    imageEditorState: {
      imageUrls: [],
    },
    timelineState: {
      layout: {
        x: 100,
        y: 100,
        width: 1000,
        height: 600,
        headerWidth: 200,
        msPerPixel: 100,
        startMs: 0,
      },
      tracks: [
        {
          id: "track1",
          clips: [
            {
              id: "1-1",
              startMs: 0,
              endMs: 1000,
            },
            {
              id: "1-2",
              startMs: 1000,
              endMs: 2000,
            },
            {
              id: "1-3",
              startMs: 3000,
              endMs: 4000,
            },
          ],
        },
        {
          id: "track2",
          clips: [
            {
              id: "2-1",
              startMs: 2500,
              endMs: 3500,
            },
          ],
        },
      ],
    },
    cameraAngleEditorState: {
      layout: {
        rect: {
          x: 100,
          y: 100,
          width: 800,
          height: 800,
        },
      },
      cameraAngle: {
        imageSourceUrl: "resources/images/피디-기본-미소.png",
        sourceRect: {
          x: 0,
          y: 0,
          width: 100,
          height: 100,
        },
        destRect: {
          x: 0,
          y: 0,
          width: 100,
          height: 100,
        },
      },
      propertyTextEditor: {
        textInput: {
          targetId: undefined,
        },
      },
      wysiwygEditor: {
        resizer: {},
      },
    },
    subtitleEditorState: {
      layout: {
        rect: {
          x: 100,
          y: 100,
          width: 400,
          height: 800,
        },
        videoSize: {
          width: 1280,
          height: 720,
        },
      },
      textInput: {},
      subtitle: {
        text: "[여기에 텍스트 입력]",
        style: {
          backgroundColor: ColorUtil.Black,
          fontColor: ColorUtil.White,
          fontSize: 24,
        },
      },
      colorInput: {
        targetId: undefined,
        hue: 0,
        saturation: 0,
        lightness: 0,
        alpha: 1,
      },
    },
  },
  render,
  {
    hotReload: {
      buildServerUrl: "ws://localhost:8080",
    },
  },
);
