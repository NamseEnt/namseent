import { signIn, signOut } from "auth-astro/client";
import FundingList from "./FundingList";
import type { Session } from "@auth/core/types";

interface FundingData {
    id: string;
    title: string;
    thumbnail: string;
    currentTickets: number;
    targetTickets: number;
}

export default function Home({
    session,
    fundings,
}: {
    session: Session | null;
    fundings: FundingData[];
}) {
    const handleLogin = () => {
        signIn("google");
    };

    const handleLogout = () => {
        signOut();
    };

    return (
        <div className="min-h-screen bg-gray-50">
            {/* 헤더 섹션 */}
            <header className="bg-white shadow-sm border-b">
                <div className="max-w-6xl mx-auto px-4 py-4 flex justify-between items-center">
                    <h1 className="text-2xl font-bold text-gray-800">
                        하연이에게 저녁을 🍽️
                    </h1>

                    {session ? (
                        <div className="flex items-center gap-4">
                            {session.user?.image && (
                                <img
                                    src={session.user.image}
                                    alt="프로필 이미지"
                                    className="w-8 h-8 rounded-full"
                                />
                            )}
                            <span className="text-sm text-gray-700">
                                {session.user?.name}님
                            </span>
                            <button
                                onClick={handleLogout}
                                className="bg-red-500 hover:bg-red-600 text-white text-sm px-4 py-2 rounded-lg transition-colors"
                            >
                                로그아웃
                            </button>
                        </div>
                    ) : (
                        <button
                            onClick={handleLogin}
                            className="bg-blue-500 hover:bg-blue-600 text-white text-sm px-4 py-2 rounded-lg transition-colors flex items-center gap-2"
                        >
                            <svg className="w-4 h-4" viewBox="0 0 24 24">
                                <path
                                    fill="currentColor"
                                    d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
                                />
                                <path
                                    fill="currentColor"
                                    d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
                                />
                                <path
                                    fill="currentColor"
                                    d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
                                />
                                <path
                                    fill="currentColor"
                                    d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
                                />
                            </svg>
                            Google로 로그인
                        </button>
                    )}
                </div>
            </header>

            {/* 메인 컨텐츠 */}
            <main className="py-8">
                <FundingList fundings={fundings} />
            </main>
        </div>
    );
}
