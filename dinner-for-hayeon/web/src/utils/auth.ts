import type { Session } from "@auth/core/types";

// Admin 사용자 목록 (Google ID 기반)
const ADMIN_GOOGLE_IDS = ["108731783381066958153"];

/**
 * 세션 정보를 기반으로 admin 권한을 체크하는 함수
 * @param session - Auth.js 세션 객체
 * @returns admin 권한이 있으면 true, 없으면 false
 */
export function checkAdmin(session: Session | null): boolean {
    if (!session?.user?.id) {
        return false;
    }

    // Google ID로 admin 체크
    return ADMIN_GOOGLE_IDS.includes(session.user.id);
}

/**
 * admin 사용자 ID 목록을 가져오는 함수
 * @returns admin Google ID 목록
 */
export function getAdminGoogleIds(): readonly string[] {
    return ADMIN_GOOGLE_IDS;
}
