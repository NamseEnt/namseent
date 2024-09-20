#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { VisualNovelStack } from "../lib/stack";
import { AudioTranscodingStack } from "../lib/AudioTranscodingStack";

const app = new cdk.App();
new VisualNovelStack(app, "VisualNovelStack", {
    env: { account: process.env.CDK_DEFAULT_ACCOUNT, region: "ap-northeast-2" },
});

new AudioTranscodingStack(app, "AudioTranscodingStack", {
    env: { account: process.env.CDK_DEFAULT_ACCOUNT, region: "ap-northeast-2" },
});
