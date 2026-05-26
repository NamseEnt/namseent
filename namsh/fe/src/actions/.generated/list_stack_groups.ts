// Auto-generated from src/actions/list_stack_groups.rs

import { z } from "zod";
import { callAction } from "@forte/react";

const StackGroupSummarySchema = z.object({
    stackHash: z.string(),
    firstSeen: z.coerce.date(),
    lastSeen: z.coerce.date(),
    count: z.number(),
    storedDumps: z.number(),
    latestAppVersion: z.string(),
    latestBuildId: z.string(),
  });

const InputSchema = z.object({
  });

const OutputSchema = z.discriminatedUnion("t", [
    z.object({
    t: z.literal("Ok"),
    groups: z.array(StackGroupSummarySchema),
  }),
    z.object({
    t: z.literal("NotLoggedIn"),
  }),
    z.object({
    t: z.literal("Error"),
    message: z.string(),
  })
  ]);

export function listStackGroups(input: z.infer<typeof InputSchema>) {
  return callAction("list_stack_groups", input, OutputSchema);
}
