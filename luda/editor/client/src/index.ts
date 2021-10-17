import { startEngine } from "namui";
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
  },
  render,
  {
    hotReload: {
      buildServerUrl: "ws://localhost:8080",
    },
  },
);
