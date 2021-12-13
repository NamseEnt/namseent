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
  const languageFontWeightFilePathMap = fontFileUrlMap[typefaceType.language];
  if (!languageFontWeightFilePathMap) {
    throw new Error(`No font file URL for ${typefaceType.language}`);
  }
  const filePath = languageFontWeightFilePathMap[typefaceType.fontWeight];
  if (!filePath) {
    throw new Error(
      `No font file URL for ${typefaceType.fontWeight} in ${typefaceType.language}`,
    );
  }
  return `${baseUrl}/${filePath}`;
}
