import { Language } from "../l10n/type";
import { TypefaceType } from "./TypefaceStorage";

const baseUrl = "/engine/resources/font";
const fontFileUrlMapPromise: Promise<{ [key in Language]: string }> = fetch(
  `${baseUrl}/map.json`,
).then((response) => response.json());

export async function getFontFileUrl(
  typefaceType: TypefaceType,
): Promise<string> {
  const fontFileUrlMap = await fontFileUrlMapPromise;
  if (!fontFileUrlMap[typefaceType.language]) {
    throw new Error(`No font file URL for ${typefaceType.language}`);
  }
  return `${baseUrl}/${fontFileUrlMap[typefaceType.language]}`;
}
