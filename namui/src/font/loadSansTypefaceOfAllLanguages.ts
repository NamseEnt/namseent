import { Language } from "../l10n/type";
import { getFontFileUrl } from "./getFontFileUrl";
import { TypefaceStorage, TypefaceType } from "./TypefaceStorage";

export async function loadSansTypefaceOfAllLanguages(
  typefaceStorage: TypefaceStorage
): Promise<void> {
  for (const language in Language) {
    const typefaceType: TypefaceType = {
      language: language as Language,
      serif: false,
    };
    const fontFileUrl = await getFontFileUrl(typefaceType);
    await typefaceStorage.loadTypeface(typefaceType, fontFileUrl);
  }
}
