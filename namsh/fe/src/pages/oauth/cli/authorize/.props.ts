// Auto-generated from src/pages/oauth/cli/authorize/mod.rs

import { z } from "zod";

export const PropsSchema = z.object({
    githubLogin: z.string(),
    redirectUri: z.string(),
    codeChallenge: z.string(),
    codeChallengeMethod: z.string(),
    state: z.string(),
    defaultLabel: z.string(),
  });

export type Props = z.infer<typeof PropsSchema>;
