import { defineAction } from "astro:actions";
import { z } from "astro:schema";
import { getSessionUser } from "@/utils/auth.ts";
import { updateUserTickets } from "../utils/db";
import { db, Funding } from "astro:db";

export const server = {
    // 티켓 수량 조정 액션 (개발환경 전용, admin 전용)
    adjustTickets: defineAction({
        input: z.object({
            amount: z.number().min(0).max(1000),
        }),
        handler: async (input, context) => {
            // 개발환경 체크
            if (import.meta.env.PROD) {
                throw new Error("이 기능은 개발환경에서만 사용할 수 있습니다.");
            }

            // 세션 체크
            const sessionUser = await getSessionUser(context.request);
            if (!sessionUser) {
                throw new Error("로그인이 필요합니다.");
            }

            // admin 권한 체크
            if (!sessionUser.admin) {
                throw new Error("관리자 권한이 필요합니다.");
            }

            try {
                await updateUserTickets(sessionUser.id, input.amount);

                console.log(
                    `티켓 조정 완료: 사용자 ${sessionUser.id}, 수량 ${input.amount}`,
                );

                return {
                    success: true,
                    message: `티켓 수량이 ${input.amount}개로 설정되었습니다.`,
                    newAmount: input.amount,
                };
            } catch (error) {
                throw new Error("티켓 수량 조정에 실패했습니다.");
            }
        },
    }),

    // 펀딩 생성 액션 (admin 전용)
    createFunding: defineAction({
        input: z.object({
            title: z.string().min(1, "제목을 입력해주세요."),
            description: z.string().optional(),
            targetTickets: z
                .number()
                .min(1, "목표 티켓 수는 1개 이상이어야 합니다."),
            thumbnailUrl: z.string().url().optional().or(z.literal("")),
            contentImageUrl: z.string().url().optional().or(z.literal("")),
        }),
        handler: async (input, context) => {
            // 세션 체크
            const sessionUser = await getSessionUser(context.request);
            if (!sessionUser) {
                throw new Error("로그인이 필요합니다.");
            }

            // admin 권한 체크
            if (!sessionUser.admin) {
                throw new Error("관리자 권한이 필요합니다.");
            }

            try {
                const fundingId = `funding_${Date.now()}`;

                await db.insert(Funding).values({
                    id: fundingId,
                    title: input.title,
                    thumbnail: input.thumbnailUrl || "",
                    contentImage: input.contentImageUrl || "",
                    currentTickets: 0,
                    targetTickets: input.targetTickets,
                });

                console.log(`펀딩 생성 완료:`, {
                    id: fundingId,
                    title: input.title,
                    createdBy: sessionUser.id,
                });

                return {
                    success: true,
                    message: `"${input.title}" 펀딩이 성공적으로 생성되었습니다.`,
                    fundingId,
                };
            } catch (error) {
                throw new Error("펀딩 생성에 실패했습니다.");
            }
        },
    }),
};
