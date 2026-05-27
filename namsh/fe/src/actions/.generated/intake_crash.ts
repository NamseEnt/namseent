// Auto-generated from src/actions/intake_crash.rs

import { z } from "zod";
import { callAction } from "@forte/react";

const CrashContextSchema = z.object({
    buildId: z.string(),
    installId: z.string(),
    sessionUptimeSec: z.number(),
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
