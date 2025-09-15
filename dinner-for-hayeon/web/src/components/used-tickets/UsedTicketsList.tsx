interface UsageHistoryItem {
    id: string;
    ticketsUsed: number;
    usedAt: Date;
    fundingId: string;
    fundingTitle: string;
    fundingThumbnail: string;
}

export default function UsedTicketsList({
    usageHistory,
}: {
    usageHistory: UsageHistoryItem[];
}) {
    const formatDate = (date: Date) => {
        return new Date(date).toLocaleDateString("ko-KR", {
            year: "numeric",
            month: "long",
            day: "numeric",
        });
    };

    const handleItemClick = (fundingId: string) => {
        window.location.href = `/funding/${fundingId}`;
    };

    return (
        <div className="container mx-auto px-4 py-8">
            {/* 헤더 */}
            <div className="mb-8">
                <h1 className="text-3xl font-bold text-gray-800 mb-2">
                    사용한 티켓 내역
                </h1>
                <p className="text-gray-600">
                    지금까지 사용한 티켓 내역을 확인하세요
                </p>
            </div>

            {/* 티켓 사용 내역 리스트 */}
            {usageHistory.length === 0 ? (
                <div className="text-center py-20">
                    <div className="text-6xl mb-6">🎫</div>
                    <h2 className="text-2xl font-semibold text-gray-800 mb-4">
                        아직 사용한 티켓이 없습니다
                    </h2>
                    <p className="text-gray-600 mb-6">
                        펀딩에 참여해서 티켓을 사용해보세요!
                    </p>
                    <button
                        onClick={() => (window.location.href = "/")}
                        className="px-6 py-3 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transition-colors"
                    >
                        펀딩 보러가기
                    </button>
                </div>
            ) : (
                <div className="space-y-4">
                    {usageHistory.map((item) => (
                        <div
                            key={item.id}
                            onClick={() => handleItemClick(item.fundingId)}
                            className="bg-white rounded-xl shadow-sm hover:shadow-md transition-all duration-200 p-6 cursor-pointer border border-gray-100 hover:border-purple-200"
                        >
                            <div className="flex items-center gap-4">
                                {/* 썸네일 이미지 */}
                                <div className="flex-shrink-0">
                                    <img
                                        src={item.fundingThumbnail}
                                        alt={item.fundingTitle}
                                        className="w-20 h-20 rounded-lg object-cover"
                                    />
                                </div>

                                {/* 내용 */}
                                <div className="flex-grow">
                                    <div className="flex justify-between items-start mb-2">
                                        <h3 className="text-lg font-semibold text-gray-800 line-clamp-2">
                                            {item.fundingTitle}
                                        </h3>
                                        <div className="flex items-center gap-1 text-purple-600 font-medium">
                                            <span className="text-2xl">🎫</span>
                                            <span className="text-lg">
                                                {item.ticketsUsed}개
                                            </span>
                                        </div>
                                    </div>

                                    <div className="flex items-center gap-4 text-sm text-gray-600">
                                        <div className="flex items-center gap-1">
                                            <span>📅</span>
                                            <span>
                                                {formatDate(item.usedAt)}
                                            </span>
                                        </div>
                                    </div>
                                </div>

                                {/* 화살표 아이콘 */}
                                <div className="flex-shrink-0 text-gray-400">
                                    <svg
                                        className="w-5 h-5"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            strokeLinecap="round"
                                            strokeLinejoin="round"
                                            strokeWidth={2}
                                            d="M9 5l7 7-7 7"
                                        />
                                    </svg>
                                </div>
                            </div>
                        </div>
                    ))}
                </div>
            )}

            {/* 총 사용 티켓 수 */}
            {usageHistory.length > 0 && (
                <div className="mt-8 bg-white rounded-xl shadow-sm p-6 border border-gray-100">
                    <div className="flex items-center justify-between">
                        <h3 className="text-lg font-semibold text-gray-800">
                            총 사용 티켓
                        </h3>
                        <div className="flex items-center gap-2 text-purple-600 font-bold text-xl">
                            <span>🎫</span>
                            <span>
                                {usageHistory.reduce(
                                    (total, item) => total + item.ticketsUsed,
                                    0,
                                )}
                                개
                            </span>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
}
