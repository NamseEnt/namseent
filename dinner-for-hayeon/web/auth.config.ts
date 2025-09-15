import Google from "@auth/core/providers/google";
import { defineConfig } from "auth-astro";
import { db, User, eq } from "astro:db";

export default defineConfig({
    providers: [
        Google({
            clientId: import.meta.env.GOOGLE_CLIENT_ID,
            clientSecret: import.meta.env.GOOGLE_CLIENT_SECRET,
        }),
    ],
    callbacks: {
        async signIn({ user, profile }) {
            const userId = profile?.sub;
            const username = user.name;
            if (!userId) {
                console.error("User ID is required", user, profile);
                return false;
            }

            if (!username) {
                console.error("Username is required", user, profile);
                return false;
            }

            await db.transaction(async (tx) => {
                const existingUser = await tx
                    .select()
                    .from(User)
                    .where(eq(User.id, userId));
                if (existingUser.length === 0) {
                    await tx.insert(User).values({
                        id: userId,
                        name: username,
                        tickets: 0,
                    });
                }
            });

            return true;
        },
        jwt({ token, account }) {
            if (account) {
                token.id = account.providerAccountId;
            }
            return token;
        },
        session({ session, token }) {
            if (session.user && token?.id) {
                session.user.id = token.id as string;
            }
            return session;
        },
    },
});
