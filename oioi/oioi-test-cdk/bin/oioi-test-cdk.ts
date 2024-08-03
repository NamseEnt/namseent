#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { OioiTestCdkStack } from "../lib/oioi-test-cdk-stack";

const app = new cdk.App();

console.log(`STACK NAME: ${process.env.STACK_NAME}`);

new OioiTestCdkStack(app, process.env.STACK_NAME!!, {
    stackName: process.env.STACK_NAME,
});
