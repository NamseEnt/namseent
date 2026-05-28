// Auto-generated from src/actions/revoke_token.rs

import { z } from "zod";
import { callAction } from "@forte/react";

const InputSchema = z.object({
    id: z.string(),
  });

const OutputSchema = z.discriminatedUnion("t", [
    z.object({
    t: z.literal("Ok"),
  }),
    z.object({
    t: z.literal("NotLoggedIn"),
  }),
    z.object({
    t: z.literal("NotFound"),
  }),
    z.object({
    t: z.literal("Error"),
    message: z.string(),
  })
  ]);

export function revokeToken(input: z.infer<typeof InputSchema>) {
  return callAction("revoke_token", input, OutputSchema);
}
