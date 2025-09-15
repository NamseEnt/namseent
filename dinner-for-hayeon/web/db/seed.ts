import { db, User, Funding, TicketUsageHistory } from "astro:db";

// https://astro.build/db/seed
export default async function seed() {
    // 먼저 사용자 데이터 시드
    await db.insert(User).values([
        {
            id: "108731783381066958153", // 실제 구글 사용자 ID
            name: "남세현",
            tickets: 50,
        },
        {
            id: "test@example.com",
            name: "테스트 사용자",
            tickets: 25,
        },
    ]);

    // 펀딩 데이터 시드
    await db.insert(Funding).values([
        {
            id: "1",
            title: "하연이의 파스타 데이트 펀딩",
            thumbnail:
                "https://images.unsplash.com/photo-1621996346565-e3dbc353d2e5?w=400&h=300&fit=crop",
            contentImage:
                "https://images.unsplash.com/photo-1551782450-17144efb9c50?w=800&h=1200&fit=crop",
            currentTickets: 15,
            targetTickets: 30,
        },
        {
            id: "2",
            title: "로맨틱 프렌치 코스 저녁식사",
            thumbnail:
                "https://images.unsplash.com/photo-1414235077428-338989a2e8c0?w=400&h=300&fit=crop",
            contentImage:
                "https://images.unsplash.com/photo-1559339352-11d035aa65de?w=800&h=1200&fit=crop",
            currentTickets: 8,
            targetTickets: 20,
        },
        {
            id: "3",
            title: "특별한 날 스테이크 하우스",
            thumbnail:
                "https://images.unsplash.com/photo-1546833999-b9f581a1996d?w=400&h=300&fit=crop",
            contentImage:
                "https://images.unsplash.com/photo-1558030006-450675393462?w=800&h=1200&fit=crop",
            currentTickets: 22,
            targetTickets: 25,
        },
        {
            id: "4",
            title: "일본 정통 가이세키 요리",
            thumbnail:
                "https://images.unsplash.com/photo-1579584425555-c3ce17fd4351?w=400&h=300&fit=crop",
            contentImage:
                "https://images.unsplash.com/photo-1617093727343-374698b1b08d?w=800&h=1200&fit=crop",
            currentTickets: 3,
            targetTickets: 15,
        },
        {
            id: "5",
            title: "홈메이드 한식 풀코스",
            thumbnail:
                "https://images.unsplash.com/photo-1498654896293-37aacf113fd9?w=400&h=300&fit=crop",
            contentImage:
                "https://images.unsplash.com/photo-1569718212165-3a8278d5f624?w=800&h=1200&fit=crop",
            currentTickets: 12,
            targetTickets: 18,
        },
        {
            id: "6",
            title: "미슐랭 스타 레스토랑 체험",
            thumbnail:
                "https://images.unsplash.com/photo-1559339352-11d035aa65de?w=400&h=300&fit=crop",
            contentImage:
                "https://images.unsplash.com/photo-1551218808-94e220e084d2?w=800&h=1200&fit=crop",
            currentTickets: 5,
            targetTickets: 50,
        },
    ]);

    // 티켓 사용 내역 시드 (admin 사용자용)
    await db.insert(TicketUsageHistory).values([
        {
            id: "1",
            userId: "108731783381066958153", // 남세현(admin) 실제 구글 ID
            fundingId: "1",
            ticketsUsed: 5,
            usedAt: new Date("2024-01-15"),
        },
        {
            id: "2",
            userId: "108731783381066958153", // 남세현(admin) 실제 구글 ID
            fundingId: "2",
            ticketsUsed: 3,
            usedAt: new Date("2024-01-20"),
        },
        {
            id: "3",
            userId: "108731783381066958153", // 남세현(admin) 실제 구글 ID
            fundingId: "3",
            ticketsUsed: 2,
            usedAt: new Date("2024-02-01"),
        },
        {
            id: "4",
            userId: "108731783381066958153", // 남세현(admin) 실제 구글 ID
            fundingId: "4",
            ticketsUsed: 7,
            usedAt: new Date("2024-02-10"),
        },
        {
            id: "5",
            userId: "108731783381066958153", // 남세현(admin) 실제 구글 ID
            fundingId: "5",
            ticketsUsed: 1,
            usedAt: new Date("2024-02-15"),
        },
    ]);
}
