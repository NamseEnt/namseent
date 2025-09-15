import { getSession as getSessionAstro } from "auth-astro/server";

// Admin 사용자 목록 (Google ID 기반)
const ADMIN_GOOGLE_IDS = ["108731783381066958153"];

export async function getSessionUser(
    request: Request,
): Promise<SessionUser | undefined> {
    const session = await getSessionAstro(request);
    if (!session?.user) {
        return;
    }
    return {
        id: session.user.id!,
        name: session.user.name!,
        email: session.user.email!,
        image: session.user.image!,
        admin: ADMIN_GOOGLE_IDS.includes(session.user.id!),
    };
}

export type SessionUser = {
    id: string;
    name: string;
    email: string;
    image: string;
    admin: boolean;
};
