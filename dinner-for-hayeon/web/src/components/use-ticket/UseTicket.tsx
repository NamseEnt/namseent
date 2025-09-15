import { useState } from "react";
import GlobalNavigation from "../common/GlobalNavigation";
import type { SessionUser } from "@/utils/auth";

interface FundingData {
    id: string;
    title: string;
    thumbnail: string;
    currentTickets: number;
    targetTickets: number;
}

export default function UseTicket({
    sessionUser,
    funding,
    userTickets,
}: {
    sessionUser: SessionUser;
    funding: FundingData;
    userTickets: number;
}) {
    const [ticketsToUse, setTicketsToUse] = useState(1);

    const handleUseTickets = () => {
        if (ticketsToUse <= 0 || ticketsToUse > userTickets) {
            alert("사용할 티켓 수를 올바르게 입력해주세요.");
            return;
        }

        // 실제 티켓 사용 로직이 들어갈 곳
        console.log(`${ticketsToUse}개의 티켓을 사용합니다.`);
        alert(`${ticketsToUse}개의 티켓을 사용했습니다!`);
    };

    const progressPercentage = Math.round(
        (funding.currentTickets / funding.targetTickets) * 100,
    );

    return (
        <div className="min-h-screen bg-gray-50">
            <GlobalNavigation sessionUser={sessionUser} />

            {/* 메인 컨텐츠 */}
            <main className="py-8">
                <div className="max-w-4xl mx-auto px-4">
                    {/* 제목 */}
                    <h1 className="text-3xl font-bold text-gray-800 mb-2">
                        티켓 사용하기
                    </h1>
                    <p className="text-gray-600 mb-8">
                        "{funding.title}"에 티켓을 사용합니다
                    </p>

                    <div className="grid md:grid-cols-2 gap-8">
                        {/* 왼쪽: 펀딩 정보 */}
                        <div>
                            <img
                                src={funding.thumbnail}
                                alt={funding.title}
                                className="w-full h-64 object-cover rounded-lg shadow-md mb-6"
                            />

                            {/* 펀딩 통계 */}
                            <div className="bg-white rounded-lg shadow-md p-6">
                                <h3 className="text-lg font-semibold text-gray-800 mb-4">
                                    펀딩 현황
                                </h3>

                                <div className="flex items-end gap-2 mb-3">
                                    <span className="text-2xl font-bold text-blue-600">
                                        {funding.currentTickets}
                                    </span>
                                    <span className="text-sm text-gray-600 mb-1">
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
                        </div>

                        {/* 오른쪽: 티켓 사용 폼 */}
                        <div className="bg-white rounded-lg shadow-md p-6">
                            <h3 className="text-lg font-semibold text-gray-800 mb-6">
                                티켓 사용
                            </h3>

                            {/* 보유 티켓 */}
                            <div className="mb-6">
                                <label className="block text-sm font-medium text-gray-700 mb-2">
                                    현재 보유 티켓
                                </label>
                                <div className="text-2xl font-bold text-green-600">
                                    {userTickets}개
                                </div>
                            </div>

                            {/* 사용할 티켓 수 입력 */}
                            <div className="mb-6">
                                <label className="block text-sm font-medium text-gray-700 mb-2">
                                    사용할 티켓 수
                                </label>
                                <input
                                    type="number"
                                    min="1"
                                    max={userTickets}
                                    value={ticketsToUse}
                                    onChange={(e) =>
                                        setTicketsToUse(Number(e.target.value))
                                    }
                                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                                />
                            </div>

                            {/* 사용하기 버튼 */}
                            <button
                                onClick={handleUseTickets}
                                disabled={
                                    ticketsToUse <= 0 ||
                                    ticketsToUse > userTickets
                                }
                                className="w-full bg-blue-500 hover:bg-blue-600 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-semibold py-3 px-6 rounded-lg transition-colors mb-6"
                            >
                                {ticketsToUse}개 티켓 사용하기
                            </button>

                            {/* 면책 고지 */}
                            <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
                                <h4 className="text-sm font-semibold text-yellow-800 mb-2">
                                    ⚠️ 중요 안내
                                </h4>
                                <p className="text-sm text-yellow-700">
                                    티켓을 한번 사용하면{" "}
                                    <strong>환불이나 철회가 불가능</strong>
                                    합니다. 신중하게 선택해주세요.
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </main>
        </div>
    );
}
