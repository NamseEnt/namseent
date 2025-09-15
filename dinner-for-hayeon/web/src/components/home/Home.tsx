import FundingList from "./FundingList";

interface FundingData {
    id: string;
    title: string;
    thumbnail: string;
    currentTickets: number;
    targetTickets: number;
}

export default function Home({
    fundings,
}: {
    fundings: FundingData[];
}) {
    return (
        <main className="py-8">
            <FundingList fundings={fundings} />
        </main>
    );
}
