import { CanvasKit, Image } from "canvaskit-wasm";
import { RenderingTree } from "..";
import { IManagerInternal } from "../managers/IManager";

export interface IImageLoadManager {
  tryLoad(url: string): Image | undefined;
}

export class ImageLoadManager implements IImageLoadManager, IManagerInternal {
  private readonly loadedMap: Map<string, Image> = new Map();
  private readonly loadingMap: Map<string, Promise<void>> = new Map();

  resetBeforeRender?: () => void;
  destroy?: () => void;
  afterRender?: (renderingTree: RenderingTree) => void;

  // non-gc canvasKit
  constructor(private readonly canvasKit: CanvasKit) {}
  tryLoad(url: string): Image | undefined {
    const image = this.loadedMap.get(url);
    if (image) {
      return image;
    }
    if (!this.loadingMap.has(url)) {
      this.loadingMap.set(url, this.startLoad(url));
    }
    return undefined;
  }
  private async startLoad(url: string): Promise<void> {
    try {
      const encodedUrl = encodeURI(url);
      const response = await fetch(encodedUrl);
      if (!response.ok) {
        console.error(
          `Failed to load image: ${encodedUrl}, status: ${
            response.status
          }, ${await response.text()}`,
        );
        return;
      }
      const imageBuffer = await response.arrayBuffer();
      const image = this.canvasKit.MakeImageFromEncoded(imageBuffer);
      if (!image) {
        console.error(`Failed to decode image: ${url}, buffer:`, imageBuffer);
        return;
      }

      this.loadedMap.set(url, image);
    } catch (error) {
      console.error(`Failed to load image: ${url}, error:`, error);
    } finally {
      // It's ok for successful loading situations too.
      setTimeout(() => {
        this.loadingMap.delete(url);
      }, 5000);
    }
  }
}
