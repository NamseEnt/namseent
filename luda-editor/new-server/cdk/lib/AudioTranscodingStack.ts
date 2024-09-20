import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import path = require("path");

export class AudioTranscodingStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        const s3Bucket = cdk.aws_s3.Bucket.fromBucketName(
            this,
            "S3Bucket",
            "visual-novel-asset",
        );

        const audioTranscodingLambda = new cdk.aws_lambda.Function(
            this,
            "audioTranscodingLambda",
            {
                code: cdk.aws_lambda.Code.fromAsset(
                    path.join(__dirname, "audio-transcoding-lambda-code"),
                ),
                handler: "function.handler",
                runtime: cdk.aws_lambda.Runtime.PROVIDED_AL2023,
                environment: {
                    BUCKET_NAME: s3Bucket.bucketName,
                },
                architecture: cdk.aws_lambda.Architecture.X86_64,
                role: new cdk.aws_iam.Role(this, "audioTranscodingLambdaRole", {
                    assumedBy: new cdk.aws_iam.ServicePrincipal(
                        "lambda.amazonaws.com",
                    ),
                    inlinePolicies: {
                        InstancePolicy: new cdk.aws_iam.PolicyDocument({
                            statements: [
                                new cdk.aws_iam.PolicyStatement({
                                    effect: cdk.aws_iam.Effect.ALLOW,
                                    actions: [
                                        "s3:DeleteObject",
                                        "s3:GetBucketLocation",
                                        "s3:GetObject",
                                        "s3:ListBucket",
                                        "s3:PutObject",
                                    ],
                                    resources: [s3Bucket.bucketArn],
                                }),
                            ],
                        }),
                    },
                }),
            },
        );

        s3Bucket.addEventNotification(
            cdk.aws_s3.EventType.OBJECT_CREATED,
            new cdk.aws_s3_notifications.LambdaDestination(
                audioTranscodingLambda,
            ),
            { prefix: "audio/before-transcode/" },
        );

        const functionUrl = audioTranscodingLambda.addFunctionUrl({
            authType: cdk.aws_lambda.FunctionUrlAuthType.NONE,
        });

        new cdk.CfnOutput(this, "FunctionUrl", {
            value: functionUrl.url,
        });
    }
}
