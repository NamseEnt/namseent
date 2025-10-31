import { Link } from "react-router";
import {
  AlertCircle,
  CheckCircle2,
  Clipboard,
  Home,
  Key,
  Trash2,
} from "lucide-react";
import { useState } from "react";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { validateApiKey } from "@/lib/gemini";
import { useSetting } from "@/lib/setting";

export default function SettingsPage() {
  const [showWarning, setShowWarning] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);
  const { setting, setSetting } = useSetting();

  const handlePasteFromClipboard = async () => {
    try {
      setIsLoading(true);
      setError(null);
      setSuccess(false);

      // Clipboard API로 텍스트 읽기
      const text = await navigator.clipboard.readText();

      if (!text || text.trim().length === 0) {
        setError(
          "클립보드가 비어있습니다. API 키를 복사한 후 다시 시도해주세요."
        );
        setIsLoading(false);
        return;
      }

      const apiKeyText = text.trim();

      // API 키 검증
      const isValid = await validateApiKey(apiKeyText);

      if (!isValid) {
        setError("유효하지 않은 API 키입니다. 다시 확인해주세요.");
        setIsLoading(false);
        return;
      }

      // 저장
      setSetting({ ...setting, apiKey: apiKeyText });
      setSuccess(true);
      setError(null);
    } catch (err) {
      console.error("클립보드 읽기 실패:", err);
      setError("클립보드 읽기에 실패했습니다. 브라우저 권한을 확인해주세요.");
    } finally {
      setIsLoading(false);
    }
  };

  const handleRemoveApiKey = () => {
    if (confirm("API 키를 제거하시겠습니까?")) {
      setSetting({ ...setting, apiKey: null });
      setSuccess(false);
      setError(null);
    }
  };

  // 스트리머 경고가 활성화되어 있으면 전체 화면을 가림
  if (showWarning) {
    return (
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-background p-4">
        <Card className="w-full max-w-md border-orange-500">
          <CardHeader>
            <div className="flex items-center gap-3 mb-2">
              <AlertCircle className="h-8 w-8 stroke-orange-500" />
              <CardTitle className="text-2xl">방송 중이신가요?</CardTitle>
            </div>
          </CardHeader>
          <CardContent className="space-y-6">
            <p className="text-lg">
              민감한 정보는 없으나 가급적 설정 중에는 화면 송출을 멈춰주세요.
            </p>
            <Button
              onClick={() => setShowWarning(false)}
              className="w-full"
              size="lg"
            >
              확인
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="container mx-auto max-w-4xl py-8 px-4">
      <div className="mb-6">
        <h1 className="text-3xl font-bold mb-2">설정</h1>
        <p className="text-muted-foreground">
          Google AI Studio API 키를 설정하세요
        </p>
      </div>

      {/* API 키 상태 */}
      <Card className="mb-6">
        <CardHeader>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Key className="h-5 w-5" />
              <CardTitle>API 키 상태</CardTitle>
            </div>
            {setting.apiKey && <Badge variant="default">설정됨</Badge>}
            {!setting.apiKey && <Badge variant="secondary">미설정</Badge>}
          </div>
        </CardHeader>
        <CardContent>
          {setting.apiKey ? (
            <div className="space-y-4">
              <div className="flex items-center gap-2 text-sm text-green-600 dark:text-green-400">
                <CheckCircle2 className="h-4 w-4" />
                <span>API 키가 설정되어 있습니다</span>
              </div>
              <Button
                onClick={handleRemoveApiKey}
                variant="destructive"
                size="sm"
              >
                <Trash2 className="h-4 w-4 mr-2" />
                API 키 제거
              </Button>
            </div>
          ) : (
            <div className="text-sm text-muted-foreground">
              API 키를 설정해주세요
            </div>
          )}
        </CardContent>
      </Card>

      {/* API 키 설정 (키가 없을 때만) */}
      {!setting.apiKey && (
        <>
          <Card className="mb-6">
            <CardHeader>
              <CardTitle>Google AI Studio API 키 발급</CardTitle>
              <CardDescription>
                아래 단계를 따라 API 키를 발급받으세요
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div>
                <Button asChild variant="outline" className="mb-4">
                  <a
                    href="https://aistudio.google.com/api-keys"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    Google AI Studio 열기
                  </a>
                </Button>
              </div>

              <Separator />

              <div className="space-y-3 text-sm">
                <p className="font-semibold">발급 절차:</p>
                <ol className="list-decimal list-inside space-y-2 ml-2">
                  <li>
                    <strong>"API 키 만들기"</strong> 버튼을 클릭합니다
                  </li>
                  <li>
                    <strong>"키 이름 지정"</strong>에 아무 이름이나 입력합니다
                    (예: 123)
                  </li>
                  <li>
                    <strong>"가져온 프로젝트 선택"</strong>
                    에서 "Create project"를 클릭하고 프로젝트 이름을
                    "yhm123"으로 입력합니다
                  </li>
                  <li>
                    <strong>"결제 설정"</strong>이{" "}
                    <Badge variant="secondary">무료 등급</Badge>
                    으로 되어있는지 확인합니다
                  </li>
                  <li>
                    오른쪽의 <strong>복사 버튼</strong>을 클릭하여 API 키를
                    클립보드에 복사합니다
                  </li>
                </ol>
              </div>
            </CardContent>
          </Card>

          <Card className="mb-6">
            <CardHeader>
              <CardTitle>API 키 등록</CardTitle>
              <CardDescription>
                복사한 API 키를 클립보드에서 가져옵니다
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <Button
                onClick={handlePasteFromClipboard}
                disabled={isLoading}
                className="w-full"
                size="lg"
              >
                <Clipboard className="h-4 w-4 mr-2" />
                {isLoading ? "검증 중..." : "클립보드에서 API 키 가져오기"}
              </Button>

              {error && (
                <Alert variant="destructive">
                  <AlertCircle className="h-4 w-4" />
                  <AlertDescription>{error}</AlertDescription>
                </Alert>
              )}

              {success && (
                <Alert className="border-green-500">
                  <CheckCircle2 className="h-4 w-4 stroke-green-500" />
                  <AlertDescription className="text-green-600 dark:text-green-400">
                    API 키가 성공적으로 설정되었습니다!
                  </AlertDescription>
                </Alert>
              )}
            </CardContent>
          </Card>
        </>
      )}

      {/* 홈으로 돌아가기 */}
      <div className="flex justify-center">
        <Button asChild variant="outline">
          <Link to="/">
            <Home className="h-4 w-4 mr-2" />
            홈으로 돌아가기
          </Link>
        </Button>
      </div>
    </div>
  );
}
