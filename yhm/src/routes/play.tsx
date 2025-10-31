import { Link } from "react-router";
import { AlertCircle } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ChatMessage, ChatInput } from "@/components/ui/chat-message";
import { useChat } from "@/lib/gemini-chat";
import { getDefaultScenario } from "@/lib/scenario";
import { useState } from "react";
import { useSetting } from "@/lib/setting";

export default function PlayPageRoute() {
  const {
    setting: { apiKey },
  } = useSetting();

  if (!apiKey) {
    return (
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-background p-4">
        <Card className="w-full max-w-md border-orange-500">
          <CardHeader>
            <div className="flex items-center gap-3 mb-2">
              <AlertCircle className="h-8 w-8 stroke-orange-500" />
              <CardTitle className="text-2xl">API Key를 설정해주세요</CardTitle>
            </div>
          </CardHeader>
          <CardContent className="space-y-6">
            <p className="text-lg">
              게임을 시작하려면 먼저 Google AI Studio API 키를 설정해야 합니다.
            </p>
            <Button asChild className="w-full" size="lg">
              <Link to="/settings">확인</Link>
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  return <PlayPage apiKey={apiKey} />;
}

function PlayPage({ apiKey }: { apiKey: string }) {
  const [inputValue, setInputValue] = useState("");
  const { chats, isModelResponsing, putUserMessage } = useChat({
    apiKey,
    scenario: getDefaultScenario(),
  });

  const handleSendMessage = () => {
    const userMessage = inputValue.trim();
    if (!userMessage || isModelResponsing) {
      return;
    }

    setInputValue("");
    putUserMessage(userMessage);
  };

  return (
    <div className="flex flex-col h-screen bg-gray-50">
      <div className="flex-1 overflow-y-auto px-4 py-6">
        <div className="max-w-4xl mx-auto">
          {chats.map((message, index) => (
            <ChatMessage
              key={index}
              role={message.role}
              content={message.content}
            />
          ))}
        </div>
      </div>

      <ChatInput
        value={inputValue}
        onChange={setInputValue}
        onSubmit={handleSendMessage}
        disabled={isModelResponsing}
        placeholder=""
      />
    </div>
  );
}
