import { GoogleGenAI } from "@google/genai";

export async function validateApiKey(apiKey: string): Promise<boolean> {
  try {
    const ai = new GoogleGenAI({ apiKey });

    const response = await ai.models.generateContent({
      model: "models/gemini-2.5-flash-lite",
      contents: "Say 'a'",
    });

    return !!(
      response.text &&
      typeof response.text === "string" &&
      response.text.length > 0
    );
  } catch (error) {
    console.error("API 키 검증 실패:", error);
    return false;
  }
}

export function createGeminiClient(apiKey: string) {
  return new GoogleGenAI({ apiKey });
}
