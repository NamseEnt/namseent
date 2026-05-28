// Auto-generated from src/actions/add_user.rs

import { z } from "zod";
import { callAction } from "@forte/react";

const InputSchema = z.object({
    githubId: z.number(),
    githubLogin: z.string(),
  });

const OutputSchema = z.discriminatedUnion("t", [
    z.object({
    t: z.literal("Ok"),
  }),
    z.object({
    t: z.literal("Unauthorized"),
  }),
    z.object({
    t: z.literal("AlreadyExists"),
  }),
    z.object({
    t: z.literal("Error"),
    message: z.string(),
  })
  ]);

export function addUser(input: z.infer<typeof InputSchema>) {
  return callAction("add_user", input, OutputSchema);
}
