// Auto-generated from src/actions/confirm_pdb_uploaded.rs

import { z } from "zod";
import { callAction } from "@forte/react";

const InputSchema = z.object({
    buildId: z.string(),
  });

const OutputSchema = z.discriminatedUnion("t", [
    z.object({
    t: z.literal("Ok"),
    size: z.number(),
  }),
    z.object({
    t: z.literal("NotLoggedIn"),
  }),
    z.object({
    t: z.literal("BuildNotFound"),
  }),
    z.object({
    t: z.literal("NotUploaded"),
  }),
    z.object({
    t: z.literal("Error"),
    message: z.string(),
  })
  ]);

export function confirmPdbUploaded(input: z.infer<typeof InputSchema>) {
  return callAction("confirm_pdb_uploaded", input, OutputSchema);
}
