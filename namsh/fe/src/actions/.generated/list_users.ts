// Auto-generated from src/actions/list_users.rs

import { z } from "zod";
import { callAction } from "@forte/react";

const UserSummarySchema = z.object({
    githubId: z.number(),
    githubLogin: z.string(),
    createdAt: z.coerce.date(),
  });

const InputSchema = z.object({
  });

const OutputSchema = z.discriminatedUnion("t", [
    z.object({
    t: z.literal("Ok"),
    users: z.array(UserSummarySchema),
  }),
    z.object({
    t: z.literal("Unauthorized"),
  }),
    z.object({
    t: z.literal("Error"),
    message: z.string(),
  })
  ]);

export function listUsers(input: z.infer<typeof InputSchema>) {
  return callAction("list_users", input, OutputSchema);
}
