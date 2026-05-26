// Auto-generated from src/pages/index/mod.rs

import { z } from "zod";

export const StackGroupRowSchema = z.object({
    stackHash: z.string(),
    firstSeen: z.coerce.date(),
    lastSeen: z.coerce.date(),
    count: z.number(),
    storedDumps: z.number(),
    latestAppVersion: z.string(),
    latestBuildId: z.string(),
  });

export type StackGroupRow = z.infer<typeof StackGroupRowSchema>;

export const PropsSchema = z.object({
    githubLogin: z.string(),
    groups: z.array(StackGroupRowSchema),
  });

export type Props = z.infer<typeof PropsSchema>;
