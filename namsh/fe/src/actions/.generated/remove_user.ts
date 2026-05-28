// Auto-generated from src/actions/remove_user.rs

import { z } from "zod";
import { callAction } from "@forte/react";

const InputSchema = z.object({
    githubId: z.number(),
  });

const OutputSchema = z.discriminatedUnion("t", [
    z.object({
    t: z.literal("Ok"),
  }),
    z.object({
    t: z.literal("Unauthorized"),
  }),
    z.object({
    t: z.literal("NotFound"),
  }),
    z.object({
    t: z.literal("Error"),
    message: z.string(),
  })
  ]);

export function removeUser(input: z.infer<typeof InputSchema>) {
  return callAction("remove_user", input, OutputSchema);
}
