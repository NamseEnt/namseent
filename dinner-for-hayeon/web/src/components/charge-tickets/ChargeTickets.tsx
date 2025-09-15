import { useState } from "react";
import GlobalNavigation from "../common/GlobalNavigation";
import type { SessionUser } from "@/utils/auth";

export default function ChargeTickets({
    session,
    userTickets = 0,
}: {
    session: Session | null;
    userTickets?: number;
}) {
    const [selectedTicketCount, setSelectedTicketCount] = useState(0);
    const [customAmount, setCustomAmount] = useState("");

    const selectTicketCount = (count: number) => {
        setSelectedTicketCount(count);
        setCustomAmount("");
    };

    const updateCustomAmount = (value: string) => {
        const numValue = parseInt(value) || 0;
        setCustomAmount(value);

        if (numValue > 0 && numValue <= 500) {
            setSelectedTicketCount(numValue);
        } else {
            setSelectedTicketCount(0);
        }
    };

    const handlePayment = () => {
        if (selectedTicketCount === 0) {
            alert("충전할 티켓 수량을 선택해주세요.");
            return;
        }

        alert(
            `현재 결제 기능은 개발 중입니다.\n선택된 수량: ${selectedTicketCount}개\n결제 금액: ${(
                selectedTicketCount * 1000
            ).toLocaleString()}원`,
        );
    };

    const ticketOptions = [1, 5, 10, 20, 50, 100];

    return (
        <div className="min-h-screen bg-gray-50">
            <GlobalNavigation session={session} userTickets={userTickets} />

            {/* 메인 컨텐츠 */}
            <main className="max-w-2xl mx-auto px-4 py-8">
                {/* 현재 보유 티켓 */}
                <div className="bg-white rounded-lg shadow-sm border p-6 mb-6">
                    <h2 className="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
                        <span className="text-2xl">💰</span>
                        현재 보유 티켓
                    </h2>
                    <div className="bg-orange-50 border border-orange-200 rounded-lg p-4">
                        <div className="flex items-center justify-center">
                            <span className="text-3xl font-bold text-orange-600">
                                {userTickets}개
                            </span>
                            <span className="text-orange-600 ml-2 text-xl">
                                🎫
                            </span>
                        </div>
                    </div>
                </div>

                {/* 충전할 티켓 수량 선택 */}
                <div className="bg-white rounded-lg shadow-sm border p-6 mb-6">
                    <h2 className="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
                        <span className="text-2xl">🛒</span>
                        충전할 티켓 수량
                    </h2>

                    <div className="space-y-4">
                        {/* 수량 선택 버튼들 */}
                        <div className="grid grid-cols-3 gap-3 mb-4">
                            {ticketOptions.map((count) => (
                                <button
                                    key={count}
                                    className={`border-2 rounded-lg p-4 transition-colors text-center ${
                                        selectedTicketCount === count &&
                                        customAmount === ""
                                            ? "border-orange-500 bg-orange-50"
                                            : "border-gray-200 hover:border-orange-300"
                                    }`}
                                    onClick={() => selectTicketCount(count)}
                                >
                                    <div className="text-lg font-semibold text-gray-800">
                                        {count}개
                                    </div>
                                    <div className="text-sm text-gray-500">
                                        {(count * 1000).toLocaleString()}원
                                    </div>
                                </button>
                            ))}
                        </div>

                        {/* 직접 입력 */}
                        <div className="border-t pt-4">
                            <label className="block text-sm font-medium text-gray-700 mb-2">
                                직접 입력 (최대 500개)
                            </label>
                            <input
                                type="number"
                                min="1"
                                max="500"
                                value={customAmount}
                                onChange={(e) =>
                                    updateCustomAmount(e.target.value)
                                }
                                className="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-transparent"
                                placeholder="충전할 티켓 수량을 입력하세요"
                            />
                        </div>
                    </div>
                </div>

                {/* 결제 정보 */}
                <div className="bg-white rounded-lg shadow-sm border p-6 mb-6">
                    <h2 className="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
                        <span className="text-2xl">💳</span>
                        결제 정보
                    </h2>

                    <div className="space-y-3">
                        <div className="flex justify-between items-center">
                            <span className="text-gray-600">선택된 티켓:</span>
                            <span className="font-semibold">
                                {selectedTicketCount}개
                            </span>
                        </div>
                        <div className="flex justify-between items-center">
                            <span className="text-gray-600">티켓 당 가격:</span>
                            <span className="font-semibold">1,000원</span>
                        </div>
                        <hr className="border-gray-200" />
                        <div className="flex justify-between items-center text-lg">
                            <span className="font-semibold text-gray-800">
                                총 결제 금액:
                            </span>
                            <span className="font-bold text-orange-600">
                                {(selectedTicketCount * 1000).toLocaleString()}
                                원
                            </span>
                        </div>
                    </div>
                </div>

                {/* 환불 안내 */}
                <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4 mb-6">
                    <h3 className="text-sm font-semibold text-yellow-800 mb-2 flex items-center gap-2">
                        <span>⚠️</span>
                        소비자보호법에 의거한 환불 안내
                    </h3>
                    <p className="text-xs text-yellow-700 leading-relaxed">
                        구매한 티켓은 구입일로부터 7일 이내에 환불이 가능합니다.
                        단, 이미 사용한 티켓은 환불이 불가능하며, 환불 시
                        수수료가 차감될 수 있습니다. 환불을 원하시는 경우
                        고객센터로 연락주시기 바랍니다.
                    </p>
                </div>

                {/* 결제 버튼 */}
                <div className="space-y-3">
                    <button
                        onClick={handlePayment}
                        disabled={selectedTicketCount === 0}
                        className="w-full bg-green-500 hover:bg-green-600 text-white font-semibold py-4 px-6 rounded-lg transition-colors disabled:bg-gray-300 disabled:cursor-not-allowed flex items-center justify-center gap-3"
                    >
                        <span>네이버페이로 결제하기</span>
                    </button>

                    <p className="text-xs text-gray-500 text-center">
                        현재 결제 기능은 개발 중입니다
                    </p>
                </div>
            </main>
        </div>
    );
}
