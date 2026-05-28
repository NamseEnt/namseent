// Auto-generated from src/actions/get_stack_group.rs

import { z } from "zod";
import { callAction } from "@forte/react";

const CrashContextSchema = z.object({
    buildId: z.string(),
    installId: z.string(),
    sessionUptimeSec: z.number(),
    logTail: z.string().optional(),
  });

const DumpSummarySchema = z.object({
    dumpId: z.string(),
    buildId: z.string(),
    uploadedAt: z.coerce.date(),
    clientIp: z.string(),
  });

const InputSchema = z.object({
    stackHash: z.string(),
  });

const OutputSchema = z.discriminatedUnion("t", [
    z.object({
    t: z.literal("Ok"),
    stackHash: z.string(),
    firstSeen: z.coerce.date(),
    lastSeen: z.coerce.date(),
    count: z.number(),
    latestContext: CrashContextSchema,
    dumps: z.array(DumpSummarySchema),
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

export function getStackGroup(input: z.infer<typeof InputSchema>) {
  return callAction("get_stack_group", input, OutputSchema);
}
