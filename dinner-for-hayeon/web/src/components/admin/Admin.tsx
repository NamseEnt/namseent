import { useState } from "react";
import type { SessionUser } from "@/utils/auth";
import TicketAdjustment from "./TicketAdjustment";
import FundingCreator from "./FundingCreator";

export default function Admin({ sessionUser }: { sessionUser: SessionUser }) {
    const [activeTab, setActiveTab] = useState("tickets");

    const tabs = [
        { id: "tickets", name: "티켓 관리", icon: "🎫" },
        { id: "funding", name: "펀딩 생성", icon: "💰" },
    ];

    return (
        <div className="max-w-6xl mx-auto px-4 py-8">
                <div className="bg-white rounded-lg shadow-sm border">
                    {/* 헤더 */}
                    <div className="border-b px-6 py-4">
                        <h1 className="text-2xl font-bold text-gray-800 flex items-center gap-2">
                            ⚙️ 관리자 페이지
                        </h1>
                        <p className="text-sm text-gray-600 mt-1">
                            시스템 관리 및 설정을 할 수 있습니다
                        </p>
                    </div>

                    {/* 탭 네비게이션 */}
                    <div className="border-b">
                        <nav className="flex">
                            {tabs.map((tab) => (
                                <button
                                    key={tab.id}
                                    onClick={() => setActiveTab(tab.id)}
                                    className={`px-6 py-4 text-sm font-medium border-b-2 transition-colors ${
                                        activeTab === tab.id
                                            ? "border-purple-500 text-purple-600 bg-purple-50"
                                            : "border-transparent text-gray-500 hover:text-gray-700 hover:bg-gray-50"
                                    }`}
                                >
                                    <span className="flex items-center gap-2">
                                        {tab.icon} {tab.name}
                                    </span>
                                </button>
                            ))}
                        </nav>
                    </div>

                    {/* 탭 컨텐츠 */}
                    <div className="p-6">
                        {activeTab === "tickets" && (
                            <TicketAdjustment session={sessionUser} />
                        )}
                        {activeTab === "funding" && <FundingCreator />}
                    </div>
                </div>
        </div>
    );
}
