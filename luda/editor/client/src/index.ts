import { ColorUtil, FontWeight, Language, startEngine } from "namui";
import { nanoid } from "nanoid";
import { getDefaultSyncBrowserItemsState } from "./cameraAngleEditor/imageBrowser/SyncBrowserItems";
import { render } from "./render";
import { createClip } from "./timeline/operations/createClip";
import { TrackType } from "./timeline/type";
import { SubtitleFontSize } from "./type";

const margin = 8;
const subtitleEditorHeight = 650;

startEngine(
  {
    imageEditorState: {
      imageUrls: [],
    },
    timelineState: {
      layout: {
        x: 0,
        y: window.innerHeight - 200,
        width: window.innerWidth,
        height: 200,
        headerWidth: 200,
        msPerPixel: 100,
        startMs: 0,
        timeRulerHeight: 20,
      },
      timelineBorderId: nanoid(),
      tracks: [
        {
          id: "track1",
          type: TrackType.subtitle,
          clips: [
            {
              id: "2-1",
              type: "subtitle",
              startMs: 2500,
              endMs: 3500,
              subtitle: {
                text: "subtitle 1-1",
                fontType: {
                  fontWeight: FontWeight.regular,
                  language: Language.ko,
                  serif: false,
                  size: SubtitleFontSize.regular,
                },
                style: {
                  background: {
                    color: ColorUtil.Black,
                  },
                  border: {
                    color: ColorUtil.Transparent,
                    width: 0,
                  },
                  color: ColorUtil.White,
                  dropShadow: {
                    color: ColorUtil.Transparent,
                    x: 0,
                    y: 0,
                  },
                },
              },
            },
          ],
        },
        {
          id: "track2",
          type: TrackType.camera,
          clips: [
            createClip({
              trackType: TrackType.camera,
              id: nanoid(),
              startMs: 0,
              endMs: 1000,
            }),
            createClip({
              trackType: TrackType.camera,
              id: nanoid(),
              startMs: 1000,
              endMs: 3000,
            }),
            createClip({
              trackType: TrackType.camera,
              id: nanoid(),
              startMs: 3000,
              endMs: 7000,
            }),
          ],
        },
      ],
    },
    cameraAngleEditorState: {
      layout: {
        rect: {
          x: 0,
          y: 0,
          width: 800,
          height: window.innerHeight - 200,
        },
        sub: {
          wysiwygEditor: {
            x: 400,
            y: 0,
            width: 400,
            height: (400 / 16) * 9,
          },
          preview: {
            x: 400,
            y: 400,
            width: 400,
            height: (400 / 16) * 9,
          },
        },
      },
      cameraAngle: {
        imageSourceUrl: "resources/images/피디-기본-미소.png",
        source01Rect: {
          x: 0.25,
          y: 0.25,
          width: 0,
          height: 0.5,
        },
        dest01Rect: {
          x: 0,
          y: 0,
          width: 1,
          height: 1,
        },
      },
      imageBrowser: {
        directoryKey: "",
        layout: {
          x: 0,
          y: 0,
          width: 200,
          height: window.innerHeight - 200,
          currentDirectoryLabel: {
            x: 20,
            y: 20,
            width: 160,
            height: 40,
          },
        },
        syncBrowserItems: getDefaultSyncBrowserItemsState(),
        imageFilenameObjects: [],
        scrollState: {
          scrollY: 0,
        },
      },
      wysiwygEditor: {
        resizer: {},
      },
    },
    subtitleEditorState: {
      layout: {
        rect: {
          x: margin,
          y: margin,
          width: 400,
          height: subtitleEditorHeight,
        },
        videoSize: {
          width: 1280,
          height: 720,
        },
      },
      textInput: {},
      subtitle: {
        text: "[여기에 텍스트 입력]",
        fontType: {
          serif: false,
          size: 24,
          language: Language.ko,
          fontWeight: FontWeight.regular,
        },
        style: {
          color: ColorUtil.White,
          background: {
            color: ColorUtil.Black,
          },
          border: {
            color: ColorUtil.Transparent,
            width: 1,
          },
          dropShadow: {
            x: 1,
            y: 1,
            color: ColorUtil.Transparent,
          },
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
