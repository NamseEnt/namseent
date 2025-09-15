import { signIn } from "auth-astro/client";
import type { SessionUser } from "@/utils/auth";

interface FundingData {
    id: string;
    title: string;
    thumbnail: string;
    currentTickets: number;
    targetTickets: number;
    contentImage: string;
}

export default function FundingDetail({
    sessionUser,
    funding,
}: {
    sessionUser: SessionUser | null;
    funding: FundingData;
}) {
    const handleUseTicket = () => {
        if (!sessionUser) {
            // 로그인이 안되어 있으면 구글 로그인 시작하고 로그인 후 티켓 사용 페이지로 리다이렉트
            signIn("google", { callbackUrl: `/use-ticket/${funding.id}` });
            return;
        }

        // 로그인이 되어 있으면 티켓 사용 페이지로 이동
        window.location.href = `/use-ticket/${funding.id}`;
    };

    const progressPercentage = Math.round(
        (funding.currentTickets / funding.targetTickets) * 100,
    );

    return (
        <main>
                {/* 상단 메인 섹션 */}
                <div className="bg-white border-b">
                    <div className="max-w-6xl mx-auto px-4 py-8">
                        <div className="grid md:grid-cols-2 gap-12">
                            {/* 왼쪽: 썸네일 */}
                            <div>
                                <img
                                    src={funding.thumbnail}
                                    alt={funding.title}
                                    className="w-full h-96 object-cover rounded-lg shadow-lg"
                                />
                            </div>

                            {/* 오른쪽: 프로젝트 정보 */}
                            <div className="flex flex-col justify-center">
                                <h1 className="text-4xl font-bold text-gray-800 mb-6 leading-tight">
                                    {funding.title}
                                </h1>

                                {/* 펀딩 통계 */}
                                <div className="mb-8">
                                    <div className="flex items-end gap-2 mb-3">
                                        <span className="text-3xl font-bold text-blue-600">
                                            {funding.currentTickets}
                                        </span>
                                        <span className="text-lg text-gray-600 mb-1">
                                            / {funding.targetTickets} 티켓
                                        </span>
                                    </div>

                                    {/* 진행률 바 */}
                                    <div className="w-full bg-gray-200 rounded-full h-2 mb-2">
                                        <div
                                            className="bg-blue-500 h-2 rounded-full transition-all duration-300"
                                            style={{
                                                width: `${Math.min(
                                                    progressPercentage,
                                                    100,
                                                )}%`,
                                            }}
                                        />
                                    </div>
                                    <div className="text-sm text-gray-500">
                                        {progressPercentage}% 달성
                                    </div>
                                </div>

                                {/* 티켓 사용하기 버튼 */}
                                <button
                                    onClick={handleUseTicket}
                                    className="w-full bg-green-500 hover:bg-green-600 text-white font-bold py-4 px-8 rounded-lg transition-colors text-lg"
                                >
                                    티켓 사용하기 🎫
                                </button>
                            </div>
                        </div>
                    </div>
                </div>

                {/* 상세 내용 섹션 */}
                <div className="py-16">
                    <div className="max-w-3xl mx-auto px-4">
                        <h2 className="text-2xl font-bold text-gray-800 mb-8 text-center">
                            프로젝트 상세
                        </h2>
                        <div className="text-center">
                            <img
                                src={funding.contentImage}
                                alt={`${funding.title} 상세 이미지`}
                                className="w-full h-auto rounded-lg shadow-md"
                            />
                        </div>
                    </div>
                </div>
        </main>
    );
}
