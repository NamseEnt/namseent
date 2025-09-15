import { defineDb, defineTable, column } from "astro:db";

const User = defineTable({
    columns: {
        id: column.text({ primaryKey: true }),
        name: column.text(),
        tickets: column.number({ default: 0 }),
        createdAt: column.date({ default: new Date() }),
    },
});

const Funding = defineTable({
    columns: {
        id: column.text({ primaryKey: true }),
        title: column.text(),
        thumbnail: column.text(),
        contentImage: column.text(),
        currentTickets: column.number(),
        targetTickets: column.number(),
        createdAt: column.date({ default: new Date() }),
    },
});

const TicketUsageHistory = defineTable({
    columns: {
        id: column.text({ primaryKey: true }),
        userId: column.text({ references: () => User.columns.id }),
        fundingId: column.text({ references: () => Funding.columns.id }),
        ticketsUsed: column.number(),
        usedAt: column.date({ default: new Date() }),
    },
});

// https://astro.build/db/config
export default defineDb({
    tables: {
        User,
        Funding,
        TicketUsageHistory,
    },
});
