import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as fs from "fs";
import * as path from "path";

export interface CloudfrontVpcOriginProps {
    vpcOriginName: string;
}

export class CloudfrontVpcOriginRemover extends Construct {
    constructor(scope: Construct, id: string, props: CloudfrontVpcOriginProps) {
        super(scope, id);

        const fn = new cdk.aws_lambda.SingletonFunction(this, "Singleton", {
            uuid: "f7d4f730-4ee1-11e8-9c2d-fa7ae01bbebc",
            code: new cdk.aws_lambda.InlineCode(
                fs.readFileSync(
                    path.join(__dirname, "../cloudfrontVpcOriginFn/index.js"),
                    { encoding: "utf-8" },
                ),
            ),
            handler: "index.handler",
            timeout: cdk.Duration.seconds(60),
            runtime: cdk.aws_lambda.Runtime.NODEJS_22_X,
        });

        const provider = new cdk.custom_resources.Provider(this, "Provider", {
            onEventHandler: fn,
        });

        new cdk.CustomResource(this, "Resource", {
            serviceToken: provider.serviceToken,
            properties: props,
        });
    }
}
