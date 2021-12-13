import { IBuildServerConnection } from "../BuildServerConnection";
import { HotReloadModule } from "./type";

type HotReloadContext = {
  hotReloadModule: HotReloadModule;
  getState: () => any;
  stopEngine: () => void;
  currentScript: HTMLScriptElement;
};

export async function setHotReload({
  buildServerConnection,
  getState,
  stopEngine,
  currentScript,
}: {
  buildServerConnection: IBuildServerConnection;
  getState: () => any;
  stopEngine: () => void;
  currentScript: HTMLScriptElement;
}): Promise<void> {
  const context: HotReloadContext = {
    hotReloadModule: createHotReloadModule(),
    getState,
    stopEngine,
    currentScript,
  };

  setHotReloadModule(context.hotReloadModule);

  connectToBuildServer({
    context,
    buildServerConnection,
  });
}

function createHotReloadModule(): HotReloadModule {
  return {};
}

function setHotReloadModule(hotReloadModule: HotReloadModule): void {
  globalThis.hotReloadModule = hotReloadModule;
}

function connectToBuildServer({
  context,
  buildServerConnection,
}: {
  context: HotReloadContext;
  buildServerConnection: IBuildServerConnection;
}): void {
  function onReload() {
    buildServerConnection.removeEventListener("reload", onReload);
    onReloadRequest(context);
  }
  buildServerConnection.addEventListener("reload", onReload);
}

function onReloadRequest(context: HotReloadContext) {
  saveStateToHotReloadModule(context);
  context.stopEngine();
  reloadScript(context);
}

function saveStateToHotReloadModule(context: HotReloadContext) {
  const state = context.getState();
  context.hotReloadModule.state = state;
}

function reloadScript(context: HotReloadContext) {
  const { currentScript } = context;
  const { src } = currentScript;
  currentScript.remove();
  const newScriptElement = document.createElement("script");
  newScriptElement.src = src;
  document.body.appendChild(newScriptElement);
}
