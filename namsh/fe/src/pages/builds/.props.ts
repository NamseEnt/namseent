// Auto-generated from src/pages/builds/mod.rs

import { z } from "zod";

export const BuildSummarySchema = z.object({
    buildId: z.string(),
    createdAt: z.coerce.date(),
    uploadedBy: z.number(),
    pdbUploaded: z.boolean(),
    pdbSize: z.number().optional(),
  });

export type BuildSummary = z.infer<typeof BuildSummarySchema>;

export const PropsSchema = z.object({
    githubLogin: z.string(),
    builds: z.array(BuildSummarySchema),
  });

export type Props = z.infer<typeof PropsSchema>;
