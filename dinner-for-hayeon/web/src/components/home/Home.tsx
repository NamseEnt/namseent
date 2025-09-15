import FundingList from "./FundingList";
import type { GlobalNavigationProps } from "../common/GlobalNavigation";

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
        <main className="py-8">
            <FundingList fundings={fundings} />
        </main>
    );
}
