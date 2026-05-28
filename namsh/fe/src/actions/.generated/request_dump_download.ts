// Auto-generated from src/actions/request_dump_download.rs

import { z } from "zod";
import { callAction } from "@forte/react";

const InputSchema = z.object({
    dumpId: z.string(),
  });

const OutputSchema = z.discriminatedUnion("t", [
    z.object({
    t: z.literal("Ok"),
    presignedGetUrl: z.string(),
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

export function requestDumpDownload(input: z.infer<typeof InputSchema>) {
  return callAction("request_dump_download", input, OutputSchema);
}
