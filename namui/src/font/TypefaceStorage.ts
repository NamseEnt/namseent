import { Typeface } from "canvaskit-wasm";
import { Language } from "../l10n/type";
import { FontWeight } from "./FontStorage";

export type TypefaceType = {
  serif: boolean;
  language: Language;
  fontWeight: FontWeight;
};
export interface ITypefaceStorage {
  getTypeface(option: TypefaceType): Typeface;
  loadTypeface(option: TypefaceType, url: string): Promise<void>;
}

export class TypefaceStorage implements ITypefaceStorage {
  private readonly typefaceMap: Map<string, Typeface> = new Map();
  constructor(typefaces?: { type: TypefaceType; typeface: Typeface }[]) {
    typefaces?.forEach(({ type, typeface }) => {
      this.typefaceMap.set(this.toKey(type), typeface);
    });
  }
  async loadTypeface(option: TypefaceType, url: string): Promise<void> {
    const key = this.toKey(option);
    if (this.typefaceMap.has(key)) {
      return;
    }
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`Failed to load typeface: ${url}`);
    }
    const buffer = await response.arrayBuffer();
    const typeface =
      CanvasKit.FontMgr.RefDefault().MakeTypefaceFromData(buffer);
    this.typefaceMap.set(key, typeface);
  }
  getTypeface(option: TypefaceType): Typeface {
    const key = this.toKey(option);
    const typeface = this.typefaceMap.get(key);
    if (!typeface) {
      throw new Error(`Typeface not found: ${key}`);
    }
    return typeface;
  }

  private toKey(typefaceType: TypefaceType): string {
    return [
      typefaceType.serif ? "serif" : "sans",
      typefaceType.language,
      typefaceType.fontWeight,
    ].join("-");
  }
}
