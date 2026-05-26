// Auto-generated from src/actions/list_tokens.rs

import { z } from "zod";
import { callAction } from "@forte/react";

const TokenSummarySchema = z.object({
    id: z.string(),
    label: z.string(),
    createdAt: z.coerce.date(),
  });

const InputSchema = z.object({
  });

const OutputSchema = z.discriminatedUnion("t", [
    z.object({
    t: z.literal("Ok"),
    tokens: z.array(TokenSummarySchema),
  }),
    z.object({
    t: z.literal("NotLoggedIn"),
  })
  ]);

export function listTokens(input: z.infer<typeof InputSchema>) {
  return callAction("list_tokens", input, OutputSchema);
}
