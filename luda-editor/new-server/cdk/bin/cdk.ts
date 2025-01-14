#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { VisualNovelStack } from "../lib/stack";
import { AudioTranscodingStack } from "../lib/AudioTranscodingStack";

const env = process.env.IS_LOCALSTACK
    ? { account: process.env.CDK_DEFAULT_ACCOUNT, region: "ap-northeast-2" }
    : {
          account: "211125547145",
          region: "ap-northeast-2",
      };

const app = new cdk.App();
new VisualNovelStack(app, "VisualNovelStack", {
    env,
});

new AudioTranscodingStack(app, "AudioTranscodingStack", {
    env,
});
