import { CanvasKit, Image } from "canvaskit-wasm";

export interface IImageLoader {
  tryLoad(url: string): Image | null;
}

export class ImageLoader implements IImageLoader {
  private readonly loadedMap: Map<string, Image> = new Map();
  private readonly loadingMap: Map<string, Promise<void>> = new Map();

  // non-gc canvasKit
  constructor(private readonly canvasKit: CanvasKit) {}
  tryLoad(url: string): Image | null {
    const image = this.loadedMap.get(url);
    if (image) {
      return image;
    }
    if (!this.loadingMap.has(url)) {
      this.loadingMap.set(url, this.startLoad(url));
    }
    return null;
  }
  private async startLoad(url: string): Promise<void> {
    try {
      const response = await fetch(url);
      if (!response.ok) {
        console.error(
          `Failed to load image: ${url}, status: ${
            response.status
          }, ${await response.text()}`
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
