import FundingList from "./FundingList";
import GlobalNavigation, {
    type GlobalNavigationProps,
} from "../common/GlobalNavigation";

interface FundingData {
    id: string;
    title: string;
    thumbnail: string;
    currentTickets: number;
    targetTickets: number;
}

export default function Home({
    gnb,
    fundings,
}: {
    gnb: GlobalNavigationProps;
    fundings: FundingData[];
}) {
    return (
        <div className="min-h-screen bg-gray-50">
            <GlobalNavigation {...gnb} />

            {/* 메인 컨텐츠 */}
            <main className="py-8">
                <FundingList fundings={fundings} />
            </main>
        </div>
    );
}
