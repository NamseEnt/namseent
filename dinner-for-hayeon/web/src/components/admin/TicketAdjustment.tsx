import { useState } from "react";
import { actions } from "astro:actions";
import type { SessionUser } from "@/utils/auth";

export default function TicketAdjustment({ sessionUser }: { sessionUser: SessionUser }) {
    const [ticketAmount, setTicketAmount] = useState("");
    const [isLoading, setIsLoading] = useState(false);
    const [message, setMessage] = useState("");

    // 개발환경인지 확인
    const isDevelopment = import.meta.env.DEV;

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!isDevelopment) {
            setMessage("개발환경에서만 사용할 수 있는 기능입니다.");
            return;
        }

        const amount = parseInt(ticketAmount);
        if (isNaN(amount) || amount < 0) {
            setMessage("올바른 숫자를 입력해주세요.");
            return;
        }

        setIsLoading(true);
        setMessage("");

        const result = await actions.adjustTickets({ amount });

        if (result.error) {
            setMessage(result.error.message);
        } else if (result.data) {
            setMessage(result.data.message);
            setTicketAmount("");
        }

        setIsLoading(false);
    };

    if (!isDevelopment) {
        return (
            <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-6">
                <div className="flex items-center gap-3 mb-2">
                    <div className="text-yellow-600">⚠️</div>
                    <h3 className="text-lg font-semibold text-yellow-800">
                        개발환경 전용 기능
                    </h3>
                </div>
                <p className="text-yellow-700">
                    티켓 수량 조정은 개발환경에서만 사용할 수 있습니다.
                </p>
            </div>
        );
    }

    return (
        <div className="space-y-6">
            <div>
                <h3 className="text-lg font-semibold text-gray-800 mb-2">
                    티켓 수량 조정
                </h3>
                <p className="text-sm text-gray-600">
                    자신의 티켓 수량을 직접 설정할 수 있습니다. (개발환경 전용)
                </p>
            </div>

            <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
                <div className="flex items-center gap-2 mb-2">
                    <div className="text-blue-600">ℹ️</div>
                    <span className="text-sm font-medium text-blue-800">
                        현재 사용자 정보
                    </span>
                </div>
                <p className="text-sm text-blue-700">
                    이름: {sessionUser.name}
                </p>
                <p className="text-sm text-blue-700">
                    ID: {sessionUser.id}
                </p>
                <p className="text-sm text-blue-700">
                    이메일: {sessionUser.email}
                </p>
            </div>

            <form onSubmit={handleSubmit} className="space-y-4">
                <div>
                    <label
                        htmlFor="ticketAmount"
                        className="block text-sm font-medium text-gray-700 mb-2"
                    >
                        설정할 티켓 수량
                    </label>
                    <input
                        type="number"
                        id="ticketAmount"
                        value={ticketAmount}
                        onChange={(e) => setTicketAmount(e.target.value)}
                        min="0"
                        placeholder="예: 50"
                        className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                        disabled={isLoading}
                    />
                </div>

                <button
                    type="submit"
                    disabled={isLoading || !ticketAmount}
                    className="bg-purple-500 hover:bg-purple-600 disabled:bg-gray-400 text-white px-4 py-2 rounded-lg transition-colors flex items-center gap-2"
                >
                    {isLoading ? (
                        <>
                            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
                            조정 중...
                        </>
                    ) : (
                        <>🎫 티켓 수량 설정</>
                    )}
                </button>
            </form>

            {message && (
                <div
                    className={`p-4 rounded-lg ${
                        message.includes("실패") || message.includes("올바른")
                            ? "bg-red-50 border border-red-200 text-red-700"
                            : "bg-green-50 border border-green-200 text-green-700"
                    }`}
                >
                    {message}
                </div>
            )}
        </div>
    );
}
