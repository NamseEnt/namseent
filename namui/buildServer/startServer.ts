import express from "express";
import ws, { WebSocket } from "ws";
import os from "os";
import isWsl from "is-wsl";
import { exec } from "child_process";
import path from "path";
import { ErrorMessage } from "../src/build/type";

const networkInterfaces = os.networkInterfaces();

export function startServer({
  port,
  onConnected,
  getBuild,
  resourcesPath,
}: {
  port: number;
  onConnected: (ws: ws) => void;
  getBuild: () => Promise<string>;
  resourcesPath: string | undefined;
}): Promise<{
  sendErrorMessages: (
    errorMessages: ErrorMessage[],
    webSocket?: WebSocket,
  ) => void;
  requestReload: () => void;
}> {
  const app = express();
  app.get("/build/bundle.js", (req, res) => {
    getBuild()
      .then((build) => {
        res.send(build);
      })
      .catch((error) => {
        console.error(error);
        res.sendStatus(500);
      });
  });
  ["/", "/index.html"].forEach((indexPath) => {
    app.get(indexPath, (req, res) => {
      res.sendFile(path.join(__dirname, "../index.html"));
    });
  });
  app.use("/engine", express.static(path.join(__dirname, "../")));
  if (resourcesPath) {
    app.use("/resources", express.static(resourcesPath));
  }

  showUrl(port);
  startBrowser(port);

  const wsServer = new ws.Server({ noServer: true });
  wsServer.on("connection", (socket) => {
    onConnected(socket);
  });

  const sendErrorMessages = (
    errorMessages: ErrorMessage[],
    webSocket?: WebSocket,
  ) => {
    const sockets = webSocket ? [webSocket] : wsServer.clients;
    sockets.forEach((socket) => {
      socket.send(
        JSON.stringify({
          type: "error",
          errorMessages,
        }),
      );
    });
  };
  const requestReload = () => {
    wsServer.clients.forEach((socket) => {
      socket.send(
        JSON.stringify({
          type: "reload",
        }),
      );
    });
  };

  return new Promise((resolve) => {
    const server = app.listen(port, () => {
      console.log(`Listening on port ${port}!`);
      resolve({
        sendErrorMessages,
        requestReload,
      });
    });

    server.on("upgrade", (request, socket, head) => {
      wsServer.handleUpgrade(request, socket as any, head, (socket) => {
        wsServer.emit("connection", socket, request);
      });
    });
  });
}

function startBrowser(port: number) {
  if (process.platform !== "win32" && !isWsl) {
    throw new Error(
      `Starting browser is not supported on this platform ${process.platform}`,
    );
  }
  exec(`cmd.exe /C start http://localhost:${port}`);
}

function showUrl(port: number) {
  Object.values(networkInterfaces).forEach((infos) => {
    infos?.forEach((details) => {
      if (!details) {
        return;
      }
      if (details.family === "IPv4") {
        console.log(`  http://${details.address}:${port}`);
      }
    });
  });
}
