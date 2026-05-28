// Auto-generated from src/actions/oauth_cli_exchange.rs

import { z } from "zod";
import { callAction } from "@forte/react";

const InputSchema = z.object({
    code: z.string(),
    codeVerifier: z.string(),
    redirectUri: z.string(),
  });

const OutputSchema = z.discriminatedUnion("t", [
    z.object({
    t: z.literal("Ok"),
    token: z.string(),
  }),
    z.object({
    t: z.literal("InvalidGrant"),
    message: z.string(),
  }),
    z.object({
    t: z.literal("Error"),
    message: z.string(),
  })
  ]);

export function oauthCliExchange(input: z.infer<typeof InputSchema>) {
  return callAction("oauth_cli_exchange", input, OutputSchema);
}
