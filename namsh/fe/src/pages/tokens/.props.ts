// Auto-generated from src/pages/tokens/mod.rs

import { z } from "zod";

export const TokenSummarySchema = z.object({
    id: z.string(),
    label: z.string(),
    createdAt: z.coerce.date(),
  });

export type TokenSummary = z.infer<typeof TokenSummarySchema>;

export const PropsSchema = z.object({
    githubId: z.number(),
    githubLogin: z.string(),
    tokens: z.array(TokenSummarySchema),
  });

export type Props = z.infer<typeof PropsSchema>;
