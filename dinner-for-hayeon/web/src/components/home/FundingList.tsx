interface FundingData {
    id: string;
    title: string;
    thumbnail: string;
    currentTickets: number;
    targetTickets: number;
}

export default function FundingList({ fundings }: { fundings: FundingData[] }) {
    return (
        <div className="w-full max-w-4xl mx-auto p-6">
            <h2 className="text-2xl font-bold text-gray-800 mb-6">
                현재 진행 중인 펀딩
            </h2>
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                {fundings.map((funding) => (
                    <a
                        key={funding.id}
                        href={`/funding/${funding.id}`}
                        className="block bg-white rounded-lg shadow-md overflow-hidden hover:shadow-lg transition-shadow cursor-pointer"
                    >
                        <img
                            src={funding.thumbnail}
                            alt={funding.title}
                            className="w-full h-48 object-cover"
                        />
                        <div className="p-4">
                            <h3 className="font-semibold text-gray-800 mb-2 line-clamp-2">
                                {funding.title}
                            </h3>
                            <div className="flex items-center justify-between text-sm text-gray-600">
                                <span>현재 티켓</span>
                                <span className="font-medium">
                                    {funding.currentTickets} /{" "}
                                    {funding.targetTickets}
                                </span>
                            </div>
                            <div className="w-full bg-gray-200 rounded-full h-2 mt-2">
                                <div
                                    className="bg-blue-500 h-2 rounded-full transition-all duration-300"
                                    style={{
                                        width: `${Math.min(
                                            (funding.currentTickets /
                                                funding.targetTickets) *
                                                100,
                                            100,
                                        )}%`,
                                    }}
                                />
                            </div>
                            <div className="text-right text-xs text-gray-500 mt-1">
                                {Math.round(
                                    (funding.currentTickets /
                                        funding.targetTickets) *
                                        100,
                                )}
                                % 달성
                            </div>
                        </div>
                    </a>
                ))}
            </div>
        </div>
    );
}
