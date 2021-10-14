import { Font, FontMgr } from "canvaskit-wasm";
import { Language } from "../l10n/type";
import { TypefaceStorage } from "./TypefaceStorage";

export type FontType = { serif: boolean; size: number; language: Language };
export interface IFontStorage {
  getFont(option: FontType): Font;
  dispose(): void;
}

export class FontStorage implements IFontStorage {
  private readonly fontTypeFontMap: Map<string, Font> = new Map();
  constructor(private readonly typefaceStorage: TypefaceStorage) {}
  dispose(): void {
    this.fontTypeFontMap.forEach((font) => {
      font.delete();
    });
    this.fontTypeFontMap.clear();
  }
  getFont(option: FontType): Font {
    const key = this.toKey(option);
    if (this.fontTypeFontMap.has(key)) {
      return this.fontTypeFontMap.get(key)!;
    }
    const font = this.createFont(option);
    this.fontTypeFontMap.set(key, font);
    return font;
  }

  private createFont(fontType: FontType): Font {
    const typeface = this.typefaceStorage.getTypeface({
      language: fontType.language,
      serif: fontType.serif,
    });
    return new CanvasKit.Font(typeface, fontType.size);
  }

  private toKey(fontType: FontType): string {
    return `${fontType.serif ? "serif" : "sans"}-${fontType.size}-${
      fontType.language
    }`;
  }
}
