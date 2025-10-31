import { Link } from "react-router";
import { Film, Settings } from "lucide-react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

export default function Home() {
  return (
    <div className="container mx-auto max-w-6xl py-8 px-4">
      {/* 헤더 */}
      <div className="mb-8 text-center">
        <h1 className="text-4xl font-bold mb-2">얀데레 호쇼 마린</h1>
        <p className="text-muted-foreground">당신, 이치미로서 몇점이야?</p>
      </div>

      {/* 갤러리 그리드 */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {/* 설정 카드 */}
        <Link to="/settings" className="group">
          <Card className="h-full transition-all hover:shadow-lg hover:border-primary cursor-pointer">
            <CardHeader>
              <div className="aspect-video bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center mb-4">
                <Settings className="h-20 w-20 text-white" />
              </div>
              <CardTitle className="group-hover:text-primary transition-colors">
                설정
              </CardTitle>
            </CardHeader>
            <CardContent>
              <CardDescription>
                Google AI Studio API 키를 설정하고 앱을 구성합니다
              </CardDescription>
            </CardContent>
          </Card>
        </Link>

        {/* 영화 카드 */}
        <Link to="/play" className="group">
          <Card className="h-full transition-all hover:shadow-lg hover:border-primary cursor-pointer">
            <CardHeader>
              <div className="aspect-video bg-gradient-to-br from-pink-500 to-rose-600 rounded-lg flex items-center justify-center mb-4">
                <Film className="h-20 w-20 text-white" />
              </div>
              <CardTitle className="group-hover:text-primary transition-colors">
                집에서 영화를 보고 나서...
              </CardTitle>
            </CardHeader>
            <CardContent>
              <CardDescription>마린과 함께한 특별한 저녁</CardDescription>
            </CardContent>
          </Card>
        </Link>
      </div>
    </div>
  );
}
