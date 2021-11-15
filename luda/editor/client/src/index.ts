import { startEngine } from "namui";
import { nanoid } from "nanoid";
import { getDefaultSyncBrowserItemsState } from "./cameraAngleEditor/imageBrowser/SyncBrowserItems";
import { render } from "./render";

fetch("/resources/sequence/sequence1.json")
  .then((response) => {
    return response.json();
  })
  .then((sequence) => {
    startEngine(
      {
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
          tracks: sequence,
        },
        cameraAngleEditorWithoutCameraAngleState: {
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
        subtitleEditorWithoutSubtitleState: {
          layout: {
            rect: {
              x: 0,
              y: 0,
              width: 800,
              height: window.innerHeight - 200,
            },
            videoSize: {
              width: 1280,
              height: 720,
            },
          },
          textInput: {},
          colorInput: {
            targetId: undefined,
            hue: 0,
            saturation: 0,
            lightness: 0,
            alpha: 1,
          },
        },
        livePlayer: {
          layout: {
            x: 800,
            y: 0,
            width: window.innerWidth - 800,
            height: window.innerHeight - 200,
          },
          state: {
            isPlaying: false,
            anchorMs: 0,
            playStartTimeMs: 0,
          },
        },
        sequenceListViewState: {
          layout: {
            rect: {
              x: 0,
              y: 0,
              width: 800,
              height: window.innerHeight - 200,
            },
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
  });
