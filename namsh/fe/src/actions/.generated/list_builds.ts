// Auto-generated from src/actions/list_builds.rs

import { z } from "zod";
import { callAction } from "@forte/react";

const BuildSummarySchema = z.object({
    buildId: z.string(),
    createdAt: z.coerce.date(),
    uploadedBy: z.number(),
    pdbUploaded: z.boolean(),
    pdbSize: z.number().optional(),
  });

const InputSchema = z.object({
  });

const OutputSchema = z.discriminatedUnion("t", [
    z.object({
    t: z.literal("Ok"),
    builds: z.array(BuildSummarySchema),
  }),
    z.object({
    t: z.literal("NotLoggedIn"),
  }),
    z.object({
    t: z.literal("Error"),
    message: z.string(),
  })
  ]);

export function listBuilds(input: z.infer<typeof InputSchema>) {
  return callAction("list_builds", input, OutputSchema);
}
