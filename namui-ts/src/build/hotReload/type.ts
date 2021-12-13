declare global {
  var hotReloadModule: HotReloadModule | undefined;
}

export type HotReloadModule = {
  state?: any;
};
