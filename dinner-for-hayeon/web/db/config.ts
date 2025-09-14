import { defineDb, defineTable, column } from 'astro:db';

// https://astro.build/db/config
export default defineDb({
  tables: {
    User: defineTable({
      columns: {
        id: column.text({ primaryKey: true }),
        name: column.text(),
        email: column.text({ unique: true }),
        image: column.text({ optional: true }),
        createdAt: column.date({ default: new Date() }),
      }
    })
  }
});
