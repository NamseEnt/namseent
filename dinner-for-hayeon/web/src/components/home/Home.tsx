import FundingList from "./FundingList";
import GlobalNavigation from "../common/GlobalNavigation";
import type { Session } from "@auth/core/types";

interface FundingData {
    id: string;
    title: string;
    thumbnail: string;
    currentTickets: number;
    targetTickets: number;
}

export default function Home({
    session,
    fundings,
}: {
    session: Session | null;
    fundings: FundingData[];
}) {
    return (
        <div className="min-h-screen bg-gray-50">
            <GlobalNavigation session={session} />

            {/* 메인 컨텐츠 */}
            <main className="py-8">
                <FundingList fundings={fundings} />
            </main>
        </div>
    );
}
