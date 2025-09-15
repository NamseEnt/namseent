import Google from "@auth/core/providers/google";
import { defineConfig } from "auth-astro";

export default defineConfig({
  providers: [
    Google({
      clientId: import.meta.env.GOOGLE_CLIENT_ID,
      clientSecret: import.meta.env.GOOGLE_CLIENT_SECRET,
    }),
  ],
  callbacks: {
    jwt({ token, user, profile }) {
      // 로그인 시 사용자 정보를 JWT 토큰에 저장
      if (user) {
        // Google OAuth에서는 profile.sub에 고유 ID가 들어있음
        token.id = profile?.sub || user.id || token.email;
      }

      return token;
    },
    session({ session, token }) {
      // 세션에 사용자 ID 포함
      if (token) {
        session.user.id = token.id as string;
      }

      return session;
    },
  },
});
