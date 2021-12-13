import { onWebSocketConnected } from "./onWebSocketConnected";
import { onWebSocketDisconnected } from "./onWebSocketDisconnected";
import { openWebSocketServer } from "./openWebSocketServer";

const port = 8001;
openWebSocketServer({
  port,
  onConnected: (socket) => {
    console.log("connected");
    onWebSocketConnected(socket);
  },
  onDisconnected: (socket) => {
    onWebSocketDisconnected(socket);
  },
});
console.log(`Server started on port ${port}`);
