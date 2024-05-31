import MainWorker from "./main-worker?worker";
import ThreadWorker from "./thread-worker?worker";

const canvas = document.createElement("canvas");
document.body.appendChild(canvas);
const offscreen = canvas.transferControlToOffscreen();

const mainWorker = new MainWorker();

mainWorker.postMessage(
  {
    canvas: offscreen,
  },
  [offscreen]
);

mainWorker.onmessage = (message) => {
  console.debug("message from main worker", message.data);
  const threadWorker = new ThreadWorker();
  threadWorker.postMessage(message.data);
};
