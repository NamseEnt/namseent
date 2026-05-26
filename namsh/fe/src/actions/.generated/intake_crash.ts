// Auto-generated from src/actions/intake_crash.rs

import { z } from "zod";
import { callAction } from "@forte/react";

const CrashContextSchema = z.object({
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

const UploadGrantSchema = z.object({
    dumpId: z.string(),
    presignedPutUrl: z.string(),
  });

const InputSchema = z.object({
    buildId: z.string(),
    stackHash: z.string(),
    context: CrashContextSchema,
  });

const OutputSchema = z.discriminatedUnion("t", [
    z.object({
    t: z.literal("Ok"),
    upload: UploadGrantSchema.optional(),
  }),
    z.object({
    t: z.literal("UnknownBuild"),
  }),
    z.object({
    t: z.literal("InvalidSignature"),
  }),
    z.object({
    t: z.literal("RateLimited"),
  }),
    z.object({
    t: z.literal("PayloadTooLarge"),
  }),
    z.object({
    t: z.literal("Error"),
    message: z.string(),
  })
  ]);

export function intakeCrash(input: z.infer<typeof InputSchema>) {
  return callAction("intake_crash", input, OutputSchema);
}
