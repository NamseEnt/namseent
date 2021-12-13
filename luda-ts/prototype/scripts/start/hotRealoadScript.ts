import { transpile } from "typescript";

const hotReloadScript = transpile(`
const websocket = new WebSocket('ws://' + location.host);
websocket.onmessage = event => {
  const message = event.data;
  console.log(message);
  if (typeof message !== 'string' || message !== 'hotReload') {
    return;
  }

  location.reload();
};
`);

export default hotReloadScript;
