declare module "*.png" {
  const url = "url";
  export default url;
}

declare module "*.jpg" {
  const url = "url";
  export default url;
}

declare module "*.mp3" {
  const url = "url";
  export default url;
}

declare module "*.webm" {
  const url = "url";
  export default url;
}

declare module "raf-manager" {
  export default class RAFManager {
    static add(
      func: (...parameter: any[]) => void,
      fps: number,
      param?: any,
    ): void;
    static remove(func: Function): void;
    static start(): void;
    static stop(): void;
  }
}
