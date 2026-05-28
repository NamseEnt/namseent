// Auto-generated from src/actions/request_pdb_upload.rs

import { z } from "zod";
import { callAction } from "@forte/react";

const InputSchema = z.object({
    buildId: z.string(),
  });

const OutputSchema = z.discriminatedUnion("t", [
    z.object({
    t: z.literal("Ok"),
    buildId: z.string(),
    hmacKeyHex: z.string(),
    pdbPresignedPutUrl: z.string(),
  }),
    z.object({
    t: z.literal("NotLoggedIn"),
  }),
    z.object({
    t: z.literal("InvalidBuildId"),
  }),
    z.object({
    t: z.literal("Error"),
    message: z.string(),
  })
  ]);

export function requestPdbUpload(input: z.infer<typeof InputSchema>) {
  return callAction("request_pdb_upload", input, OutputSchema);
}
