import { Language } from "../l10n/type";
import { FontWeight } from "./FontStorage";
import { getFontFileUrl } from "./getFontFileUrl";
import { TypefaceStorage, TypefaceType } from "./TypefaceStorage";

export async function loadSansTypefaceOfAllLanguages(
  typefaceStorage: TypefaceStorage,
): Promise<void> {
  for (const language in Language) {
    await Promise.all(
      Object.values(FontWeight).map(async (fontWeight) => {
        const typefaceType: TypefaceType = {
          language: language as Language,
          serif: false,
          fontWeight,
        };
        const fontFileUrl = await getFontFileUrl(typefaceType);
        await typefaceStorage.loadTypeface(typefaceType, fontFileUrl);
      }),
    );
  }
}
