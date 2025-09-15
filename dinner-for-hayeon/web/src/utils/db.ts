import { db, User, eq } from "astro:db";

export async function getUserTickets(userId: string): Promise<number> {
    const userResults = await db.select().from(User).where(eq(User.id, userId));
    return userResults[0]?.tickets || 0;
}

export async function updateUserTickets(userId: string, tickets: number) {
    await db.update(User).set({ tickets }).where(eq(User.id, userId));
}
