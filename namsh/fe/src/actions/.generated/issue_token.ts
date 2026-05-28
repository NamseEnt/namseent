// Auto-generated from src/actions/issue_token.rs

import { z } from "zod";
import { callAction } from "@forte/react";

const InputSchema = z.object({
    label: z.string(),
  });

const OutputSchema = z.discriminatedUnion("t", [
    z.object({
    t: z.literal("Ok"),
    id: z.string(),
    token: z.string(),
  }),
    z.object({
    t: z.literal("NotLoggedIn"),
  }),
    z.object({
    t: z.literal("Error"),
    message: z.string(),
  })
  ]);

export function issueToken(input: z.infer<typeof InputSchema>) {
  return callAction("issue_token", input, OutputSchema);
}
