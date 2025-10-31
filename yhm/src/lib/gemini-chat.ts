import { GoogleGenAI } from "@google/genai";
import type { Scenario } from "./scenario";
import { useEffect, useMemo, useState } from "react";

export type ChatMessage = {
  role: "user" | "model";
  content: string;
};

export function useChat({
  apiKey,
  scenario,
}: {
  apiKey: string;
  scenario: Scenario;
}) {
  const client = useMemo(() => new GoogleGenAI({ apiKey }), [apiKey]);
  const [chats, setChats] = useState<ChatMessage[]>([]);
  const [isModelResponsing, setIsModelResponsing] = useState(false);

  useEffect(() => {
    (async () => {
      setIsModelResponsing(true);
      try {
        const response = await putPrompt(scenario.firstMessagePrompt);
        setChats((prev) => [...prev, { role: "model", content: response }]);
      } finally {
        setIsModelResponsing(false);
      }
    })();
  }, []);

  async function putPrompt(text: string) {
    const response = await client.models.generateContent({
      model: "models/gemini-2.5-pro",
      contents: [
        ...chats.map((chat) => ({
          role: chat.role,
          parts: [
            {
              text: chat.content,
            },
          ],
        })),
        {
          role: "user",
          parts: [
            {
              text,
            },
          ],
        },
      ],
      config: {
        thinkingConfig: {
          thinkingBudget: 128,
        },
        systemInstruction: scenario.systemInstruction,
      },
    });
    return response.text || "";
  }

  async function putUserMessage(text: string) {
    setChats((prev) => [...prev, { role: "user", content: text }]);
    setIsModelResponsing(true);
    try {
      const response = await putPrompt(text);
      setChats((prev) => [...prev, { role: "model", content: response }]);
    } finally {
      setIsModelResponsing(false);
    }
  }

  return {
    chats,
    isModelResponsing,
    putUserMessage,
  };
}
