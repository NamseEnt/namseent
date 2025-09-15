import { useState } from "react";
import { actions } from "astro:actions";

interface FundingForm {
    title: string;
    description: string;
    targetTickets: number;
    thumbnailUrl: string;
    contentImageUrl: string;
}

export default function FundingCreator() {
    const [formData, setFormData] = useState<FundingForm>({
        title: "",
        description: "",
        targetTickets: 10,
        thumbnailUrl: "",
        contentImageUrl: "",
    });
    const [isLoading, setIsLoading] = useState(false);
    const [message, setMessage] = useState("");

    const handleInputChange = (field: keyof FundingForm, value: string | number) => {
        setFormData(prev => ({
            ...prev,
            [field]: value
        }));
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!formData.title.trim()) {
            setMessage("펀딩 제목을 입력해주세요.");
            return;
        }

        if (formData.targetTickets <= 0) {
            setMessage("목표 티켓 수는 1개 이상이어야 합니다.");
            return;
        }

        setIsLoading(true);
        setMessage("");

        const result = await actions.createFunding({
            title: formData.title,
            description: formData.description,
            targetTickets: formData.targetTickets,
            thumbnailUrl: formData.thumbnailUrl,
            contentImageUrl: formData.contentImageUrl
        });

        if (result.error) {
            setMessage(result.error.message);
        } else if (result.data) {
            setMessage(result.data.message);

            // 폼 초기화
            setFormData({
                title: "",
                description: "",
                targetTickets: 10,
                thumbnailUrl: "",
                contentImageUrl: "",
            });
        }

        setIsLoading(false);
    };

    const sampleImages = [
        "https://images.unsplash.com/photo-1621996346565-e3dbc353d2e5?w=400&h=300&fit=crop",
        "https://images.unsplash.com/photo-1414235077428-338989a2e8c0?w=400&h=300&fit=crop",
        "https://images.unsplash.com/photo-1546833999-b9f581a1996d?w=400&h=300&fit=crop",
        "https://images.unsplash.com/photo-1579584425555-c3ce17fd4351?w=400&h=300&fit=crop",
    ];

    return (
        <div className="space-y-6">
            <div>
                <h3 className="text-lg font-semibold text-gray-800 mb-2">
                    새 펀딩 생성
                </h3>
                <p className="text-sm text-gray-600">
                    새로운 저녁 펀딩을 생성할 수 있습니다.
                </p>
            </div>

            <form onSubmit={handleSubmit} className="space-y-6">
                {/* 제목 */}
                <div>
                    <label htmlFor="title" className="block text-sm font-medium text-gray-700 mb-2">
                        펀딩 제목 *
                    </label>
                    <input
                        type="text"
                        id="title"
                        value={formData.title}
                        onChange={(e) => handleInputChange("title", e.target.value)}
                        placeholder="예: 하연이의 특별한 이탈리안 디너"
                        className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                        disabled={isLoading}
                    />
                </div>

                {/* 설명 */}
                <div>
                    <label htmlFor="description" className="block text-sm font-medium text-gray-700 mb-2">
                        펀딩 설명
                    </label>
                    <textarea
                        id="description"
                        value={formData.description}
                        onChange={(e) => handleInputChange("description", e.target.value)}
                        placeholder="펀딩에 대한 자세한 설명을 입력해주세요..."
                        rows={4}
                        className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                        disabled={isLoading}
                    />
                </div>

                {/* 목표 티켓 수 */}
                <div>
                    <label htmlFor="targetTickets" className="block text-sm font-medium text-gray-700 mb-2">
                        목표 티켓 수 *
                    </label>
                    <input
                        type="number"
                        id="targetTickets"
                        value={formData.targetTickets}
                        onChange={(e) => handleInputChange("targetTickets", parseInt(e.target.value) || 0)}
                        min="1"
                        className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                        disabled={isLoading}
                    />
                </div>

                {/* 썸네일 이미지 URL */}
                <div>
                    <label htmlFor="thumbnailUrl" className="block text-sm font-medium text-gray-700 mb-2">
                        썸네일 이미지 URL
                    </label>
                    <input
                        type="url"
                        id="thumbnailUrl"
                        value={formData.thumbnailUrl}
                        onChange={(e) => handleInputChange("thumbnailUrl", e.target.value)}
                        placeholder="https://example.com/thumbnail.jpg"
                        className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                        disabled={isLoading}
                    />
                    <div className="mt-2">
                        <p className="text-xs text-gray-500 mb-2">빠른 선택:</p>
                        <div className="flex gap-2 flex-wrap">
                            {sampleImages.map((url, index) => (
                                <button
                                    key={index}
                                    type="button"
                                    onClick={() => handleInputChange("thumbnailUrl", url)}
                                    className="text-xs px-2 py-1 bg-gray-100 hover:bg-gray-200 rounded transition-colors"
                                    disabled={isLoading}
                                >
                                    샘플 {index + 1}
                                </button>
                            ))}
                        </div>
                    </div>
                </div>

                {/* 컨텐츠 이미지 URL */}
                <div>
                    <label htmlFor="contentImageUrl" className="block text-sm font-medium text-gray-700 mb-2">
                        컨텐츠 이미지 URL
                    </label>
                    <input
                        type="url"
                        id="contentImageUrl"
                        value={formData.contentImageUrl}
                        onChange={(e) => handleInputChange("contentImageUrl", e.target.value)}
                        placeholder="https://example.com/content.jpg"
                        className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                        disabled={isLoading}
                    />
                </div>

                {/* 미리보기 */}
                {(formData.thumbnailUrl || formData.title) && (
                    <div className="bg-gray-50 border border-gray-200 rounded-lg p-4">
                        <h4 className="text-sm font-medium text-gray-700 mb-3">미리보기</h4>
                        <div className="bg-white rounded-lg border p-4 max-w-sm">
                            {formData.thumbnailUrl && (
                                <img
                                    src={formData.thumbnailUrl}
                                    alt="썸네일 미리보기"
                                    className="w-full h-32 object-cover rounded-lg mb-3"
                                    onError={(e) => {
                                        e.currentTarget.style.display = 'none';
                                    }}
                                />
                            )}
                            <h5 className="font-medium text-gray-800 mb-2">
                                {formData.title || "제목 없음"}
                            </h5>
                            <div className="flex items-center gap-2 text-sm text-gray-600">
                                <span>목표: {formData.targetTickets}개</span>
                            </div>
                        </div>
                    </div>
                )}

                <button
                    type="submit"
                    disabled={isLoading || !formData.title.trim()}
                    className="w-full bg-purple-500 hover:bg-purple-600 disabled:bg-gray-400 text-white px-4 py-3 rounded-lg transition-colors flex items-center justify-center gap-2 font-medium"
                >
                    {isLoading ? (
                        <>
                            <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-white"></div>
                            펀딩 생성 중...
                        </>
                    ) : (
                        <>💰 펀딩 생성하기</>
                    )}
                </button>
            </form>

            {message && (
                <div className={`p-4 rounded-lg ${
                    message.includes("실패") || message.includes("입력") || message.includes("이상")
                        ? "bg-red-50 border border-red-200 text-red-700"
                        : "bg-green-50 border border-green-200 text-green-700"
                }`}>
                    {message}
                </div>
            )}
        </div>
    );
}