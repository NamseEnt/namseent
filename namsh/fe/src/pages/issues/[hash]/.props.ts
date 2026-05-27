// Auto-generated from src/pages/issues/[hash]/mod.rs

import { z } from "zod";

export const CrashContextSchema = z.object({
    buildId: z.string(),
    installId: z.string(),
    sessionUptimeSec: z.number(),
    logTail: z.string().optional(),
  });

export type CrashContext = z.infer<typeof CrashContextSchema>;

export const DumpSummarySchema = z.object({
    dumpId: z.string(),
    buildId: z.string(),
    uploadedAt: z.coerce.date(),
    clientIp: z.string(),
  });

export type DumpSummary = z.infer<typeof DumpSummarySchema>;

export const PropsSchema = z.discriminatedUnion("t", [
    z.object({
    t: z.literal("Ok"),
    githubLogin: z.string(),
    stackHash: z.string(),
    firstSeen: z.coerce.date(),
    lastSeen: z.coerce.date(),
    count: z.number(),
    latestContext: CrashContextSchema,
    dumps: z.array(DumpSummarySchema),
  }),
    z.object({
    t: z.literal("NotFound"),
    githubLogin: z.string(),
    stackHash: z.string(),
  })
  ]);

export type Props = z.infer<typeof PropsSchema>;
