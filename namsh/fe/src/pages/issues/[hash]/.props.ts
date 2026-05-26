// Auto-generated from src/pages/issues/[hash]/mod.rs

import { z } from "zod";

export const CrashContextSchema = z.object({
    appVersion: z.string(),
    buildId: z.string(),
    os: z.string(),
    osVersion: z.string(),
    arch: z.string(),
    cpu: z.string().optional(),
    gpuAdapter: z.string().optional(),
    gpuDriver: z.string().optional(),
    locale: z.string().optional(),
    installId: z.string(),
    sessionUptimeSec: z.number(),
    errorMessage: z.string().optional(),
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
